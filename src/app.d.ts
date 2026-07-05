// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
  namespace App {
    // interface Error {}
    // interface Locals {}
    // interface PageData {}
    // interface PageState {}
    // interface Platform {}
  }

  // Plaid Link is loaded at runtime from cdn.plaid.com via app.html.
  interface PlaidLinkHandler {
    open: () => void;
    exit: (opts?: { force?: boolean }) => void;
    destroy: () => void;
  }

  interface PlaidLinkOptions {
    token: string;
    onSuccess: (public_token: string, metadata: unknown) => void;
    onLoad?: () => void;
    onExit?: (err: unknown, metadata: unknown) => void;
    onEvent?: (eventName: string, metadata: unknown) => void;
  }

  const Plaid: {
    create: (options: PlaidLinkOptions) => PlaidLinkHandler;
  };

  interface Window {
    Plaid: typeof Plaid;
  }
}

export {};
