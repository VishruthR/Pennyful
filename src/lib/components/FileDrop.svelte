<!-- @component
  A file picker component using Tauri's native file dialog.
  Accepts configurable file types and supports single or multiple file selection.
-->

<script lang="ts">
  import Icon from "@iconify/svelte";
  import { open } from "@tauri-apps/plugin-dialog";

  interface Props {
    onSelect: (paths: string[]) => void;
    acceptedTypes: string[];
    multiple?: boolean;
    filterName?: string;
  }

  let { 
    onSelect, 
    acceptedTypes, 
    multiple = false,
    filterName = "Files"
  }: Props = $props();

  let selectedPaths = $state<string[]>([]);

  const displayFormats = $derived(
    acceptedTypes.map(t => t.replace('.', '')).join(', ')
  );

  // Convert acceptedTypes like [".csv", ".txt"] to extensions ["csv", "txt"]
  const extensions = $derived(
    acceptedTypes.map(t => t.replace('.', ''))
  );

  async function openFilePicker() {
    const result = await open({
      multiple,
      filters: [{ name: filterName, extensions }],
    });

    if (!result) {
      return;
    }
    
    if (multiple && Array.isArray(result)) {
      selectedPaths = result;
    } else if (typeof result === 'string') {
      selectedPaths = [result];
    }
    onSelect(selectedPaths);
  }

  function getFileName(path: string): string {
    return path.split('/').pop() || path;
  }

  function removeFile(index: number) {
    selectedPaths = selectedPaths.filter((_, i) => i !== index);
    onSelect(selectedPaths);
  }
</script>

{#if selectedPaths.length === 0}
  <button type="button" class="file-zone drop-zone" onclick={openFilePicker}>
    <Icon icon="lets-icons:upload" width={64} height={64} class="upload-icon" />
    <p class="drop-text paragraph">
      Click here to <span class="browse-link">browse files</span> from your device
    </p>
    <p class="formats-text">Supported formats: {displayFormats}</p>
  </button>
{:else}
  <div class="file-zone files-zone">
    {#each selectedPaths as filePath, index}
      <div class="file-item">
        <Icon icon="mdi:file-outline" width={24} height={24} class="file-icon" />
        <div class="file-info">
          <span class="file-name paragraph">{getFileName(filePath)}</span>
          <span class="file-path">{filePath}</span>
        </div>
        <button
          type="button"
          class="remove-btn"
          onclick={() => removeFile(index)}
          aria-label="Remove {getFileName(filePath)}"
        >
          &times;
        </button>
      </div>
    {/each}
    <button type="button" class="change-file-btn" onclick={openFilePicker}>
      Change file
    </button>
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

  .drop-zone:focus {
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

  /* Files zone specific styles */
  .files-zone {
    gap: 12px;
    padding: 24px 32px;
  }

  .file-item {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .file-item :global(.file-icon) {
    color: var(--grey-300);
    flex-shrink: 0;
  }

  .file-info {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-width: 0;
  }

  .file-name {
    color: var(--grey-500);
    font-weight: 500;
  }

  .file-path {
    font-size: 12px;
    color: var(--grey-300);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
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

  .change-file-btn {
    align-self: flex-start;
    background: none;
    border: none;
    color: var(--blue-100);
    cursor: pointer;
    font-size: 14px;
    padding: 4px 0;
    transition: color 0.15s ease;
  }

  .change-file-btn:hover {
    color: var(--blue-200, var(--grey-500));
  }
</style>
