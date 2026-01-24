<!-- @component
  A file input component.
  Accepts configurable file types and supports single or multiple file uploads.
-->

<script lang="ts">
  import Icon from "@iconify/svelte";

  interface Props {
    onUpload: (files: File[]) => void;
    acceptedTypes: string[];
    multiple?: boolean;
  }

  let { onUpload, acceptedTypes, multiple = false }: Props = $props();

  let files = $state<File[]>([]);
  let inputRef: HTMLInputElement | undefined = $state();

  const MAX_FILE_SIZE = 1024 * 1024 * 1024; // 1GB

  const acceptString = $derived(acceptedTypes.join(','));
  const displayFormats = $derived(
    acceptedTypes.map(t => t.replace('.', '')).join(', ')
  );

  function isValidFile(file: File): boolean {
    if (file.size > MAX_FILE_SIZE) return false;
    
    // Simple file extension check
    const fileName = file.name.toLowerCase();
    return acceptedTypes.some(ext => fileName.endsWith(ext.toLowerCase()));
  }

  function handleInputChange(e: Event) {
    const target = e.target as HTMLInputElement;
    const newFiles = target.files;
    
    if (!newFiles) return;

    const validFiles = Array.from(newFiles).filter(isValidFile);
    
    if (validFiles.length === 0) return;

    if (multiple) {
      files = [...files, ...validFiles];
    } else {
      files = [validFiles[0]];
    }

    onUpload(files);
  }

  function removeFile(index: number) {
    files = files.filter((_, i) => i !== index);
    onUpload(files);
    
    if (inputRef) {
      inputRef.value = '';
    }
  }
</script>

{#if files.length === 0}
  <label class="drop-zone">
    <input
      bind:this={inputRef}
      type="file"
      accept={acceptString}
      {multiple}
      class="visually-hidden"
      onchange={handleInputChange}
    />
    <Icon icon="lets-icons:upload" width={64} height={64} class="upload-icon" />
    <p class="drop-text paragraph">
      Click here to <span class="browse-link">browse files</span> from your device
    </p>
    <p class="formats-text">Supported formats: {displayFormats}</p>
  </label>
{:else}
  <div class="files-zone">
    {#each files as file, index}
      <div class="file-item">
        <Icon icon="mdi:file-outline" width={24} height={24} class="file-icon" />
        <span class="file-name paragraph">{file.name}</span>
        <button
          type="button"
          class="remove-btn"
          onclick={() => removeFile(index)}
          aria-label="Remove {file.name}"
        >
          &times;
        </button>
      </div>
    {/each}
  </div>
{/if}

<style>
  .drop-zone {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 16px;
    padding: 48px 32px;
    border: 2px dashed var(--grey-200);
    border-radius: 16px;
    background-color: var(--pure-white);
    cursor: pointer;
    transition: border-color 0.15s ease, background-color 0.15s ease;
  }

  .drop-zone:hover {
    border-color: var(--grey-300);
    background-color: var(--blue-50);
  }

  .drop-zone:focus-within {
    outline: 2px solid var(--grey-500);
    outline-offset: 2px;
  }

  .drop-zone :global(.upload-icon) {
    color: var(--grey-300);
  }

  .drop-text {
    margin: 0;
    color: var(--grey-500);
    text-align: center;
  }

  .browse-link {
    color: var(--blue-100);
  }

  .formats-text {
    margin: 0;
    font-size: 12px;
    color: var(--grey-300);
  }

  .files-zone {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 24px 32px;
    border: 2px dashed var(--grey-200);
    border-radius: 16px;
    background-color: var(--pure-white);
  }

  .file-item {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .file-item :global(.file-icon) {
    color: var(--grey-300);
    flex-shrink: 0;
  }

  .file-name {
    color: var(--grey-500);
    flex: 1;
  }

  .remove-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    background: none;
    border: none;
    font-size: 18px;
    color: var(--grey-300);
    cursor: pointer;
    padding: 2px;
    line-height: 1;
    transition: color 0.15s ease;
  }

  .remove-btn:hover {
    color: var(--grey-500);
  }

  .visually-hidden {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
  }
</style>
