<script lang="ts">
  // Plaid deep-link returns (pennyful://plaid-complete) are handled in Rust
  // (src-tauri/src/lib.rs) so the token exchange runs in exactly one place.
  import '../app.css';
</script>

<slot />
