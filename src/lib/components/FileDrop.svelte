<!-- @component
  A file input component.
  Accepts configurable file types and supports single or multiple file uploads.
-->

<script lang="ts">
  import Icon from "@iconify/svelte";

  export interface FileError {
    file: File;
    reason: 'size' | 'type';
  }

  interface Props {
    onUpload: (files: File[]) => void;
    onError?: (errors: FileError[]) => void;
    acceptedTypes: string[];
    multiple?: boolean;
    maxFileSize?: number;
  }

  let { 
    onUpload, 
    onError,
    acceptedTypes, 
    multiple = false,
    maxFileSize = 10 * 1024 * 1024 // 10MB default
  }: Props = $props();

  let files = $state<File[]>([]);
  let inputRef: HTMLInputElement | undefined = $state();
  let errorMessage = $state<string | null>(null);

  const acceptString = $derived(acceptedTypes.join(','));
  const displayFormats = $derived(
    acceptedTypes.map(t => t.replace('.', '')).join(', ')
  );

  function isValidType(file: File): boolean {
    const fileName = file.name.toLowerCase();
    return acceptedTypes.some(ext => fileName.endsWith(ext.toLowerCase()));
  }

  function validateFiles(fileList: File[]): { valid: File[]; errors: FileError[] } {
    const valid: File[] = [];
    const errors: FileError[] = [];

    for (const file of fileList) {
      if (file.size > maxFileSize) {
        errors.push({ file, reason: 'size' });
      } else if (!isValidType(file)) {
        errors.push({ file, reason: 'type' });
      } else {
        valid.push(file);
      }
    }

    return { valid, errors };
  }

  function processFiles(newFiles: File[]) {
    const { valid, errors } = validateFiles(newFiles);

    errorMessage = null;
    if (errors.length > 0) {
      const sizeErrors = errors.filter(e => e.reason === 'size');
      const typeErrors = errors.filter(e => e.reason === 'type');
      
      const messages: string[] = [];
      if (sizeErrors.length > 0) {
        const maxMB = Math.round(maxFileSize / (1024 * 1024));
        messages.push(`${sizeErrors.length} file(s) exceed ${maxMB}MB limit`);
      }
      if (typeErrors.length > 0) {
        messages.push(`${typeErrors.length} file(s) have unsupported format`);
      }
      errorMessage = messages.join('. ');
      
      onError?.(errors);
    }

    if (valid.length === 0) return;

    if (multiple) {
      files = [...files, ...valid];
    } else {
      files = [valid[0]];
    }

    onUpload(files);
  }

  function handleInputChange(e: Event) {
    const target = e.target as HTMLInputElement;
    const newFiles = target.files;
    
    if (!newFiles) return;
    processFiles(Array.from(newFiles));
  }

  function removeFile(index: number) {
    files = files.filter((_, i) => i !== index);
    errorMessage = null;
    onUpload(files);
    
    if (inputRef) {
      inputRef.value = '';
    }
  }
</script>

{#if files.length === 0}
  <label class="file-zone drop-zone">
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
    {#if errorMessage}
      <p class="error-text">{errorMessage}</p>
    {/if}
  </label>
{:else}
  <div class="file-zone files-zone">
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
    {#if errorMessage}
      <p class="error-text">{errorMessage}</p>
    {/if}
  </div>
{/if}

<style>
  /* Shared base styles for drop and files zones */
  .file-zone {
    width: 100%;
    display: flex;
    flex-direction: column;
    border: 2px dashed var(--grey-200);
    border-radius: 16px;
    background-color: var(--pure-white);
    transition: border-color 0.15s ease, background-color 0.15s ease;
  }

  /* Drop zone specific styles */
  .drop-zone {
    align-items: center;
    justify-content: center;
    gap: 16px;
    padding: 48px 32px;
    cursor: pointer;
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

  .error-text {
    margin: 0;
    font-size: 14px;
    color: var(--loss-red);
  }

  /* Files zone specific styles */
  .files-zone {
    gap: 8px;
    padding: 24px 32px;
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
</style>
