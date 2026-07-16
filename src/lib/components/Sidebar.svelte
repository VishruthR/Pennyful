<!-- @component
  Global app sidebar with brand header, primary navigation, and a settings
  link pinned to the bottom. Can be collapsed to an icon-only rail; the
  collapsed state is persisted across sessions.
-->

<script lang="ts">
  import Icon from "@iconify/svelte";
  import { page } from "$app/state";
  import { browser } from "$app/environment";

  interface NavItem {
    label: string;
    href: string;
    icon: string;
  }

  const navItems: NavItem[] = [
    { label: "Home", href: "/", icon: "mdi:home-outline" },
    { label: "Add Accounts", href: "/plaid/link", icon: "mdi:account-outline" },
    { label: "Categories", href: "/categories", icon: "mdi:tag-outline" },
  ];

  const settingsItem: NavItem = {
    label: "Settings",
    href: "/settings",
    icon: "mdi:cog-outline",
  };

  const STORAGE_KEY = "sidebar-collapsed";

  // browser tells us if the code is running in a browser. All localStorage uses must be gated by first checking browser
  let collapsed = $state(
    browser && localStorage.getItem(STORAGE_KEY) === "true"
  );

  $effect(() => {
    if (browser) {
      localStorage.setItem(STORAGE_KEY, String(collapsed));
    }
  });

  function isActive(href: string): boolean {
    const path = page.url.pathname;
    if (href === "/") {
      return path === "/";
    }
    return path === href || path.startsWith(`${href}/`);
  }
</script>

<aside class="sidebar" class:collapsed>
  <div class="brand">
    {#if !collapsed}
      <span class="brand-name h3">Pennyful</span>
    {/if}
    <button
      class="collapse-toggle"
      onclick={() => (collapsed = !collapsed)}
      title={collapsed ? "Expand sidebar" : "Collapse sidebar"}
      aria-label={collapsed ? "Expand sidebar" : "Collapse sidebar"}
    >
      <Icon icon="boxicons:sidebar" width={22} height={22} />
    </button>
  </div>

  <nav class="nav">
    {#each navItems as item}
      <a
        class="nav-item paragraph"
        class:active={isActive(item.href)}
        href={item.href}
        title={collapsed ? item.label : undefined}
      >
        <Icon icon={item.icon} width={22} height={22} />
        {#if !collapsed}
          <span class="nav-label">{item.label}</span>
        {/if}
      </a>
    {/each}
  </nav>

  <div class="footer">
    <a
      class="nav-item paragraph"
      class:active={isActive(settingsItem.href)}
      href={settingsItem.href}
      title={collapsed ? settingsItem.label : undefined}
    >
      <Icon icon={settingsItem.icon} width={22} height={22} />
      {#if !collapsed}
        <span class="nav-label">{settingsItem.label}</span>
      {/if}
    </a>
  </div>
</aside>

<style>
  .sidebar {
    display: flex;
    flex-direction: column;
    width: 260px;
    height: 100vh;
    flex-shrink: 0;
    background-color: var(--bg-white);
    border-right: 2px solid var(--grey-500);
    transition: width 0.15s ease;
  }

  .sidebar.collapsed {
    width: 72px;
  }

  .brand {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 73px;
    padding: 0 20px;
    border-bottom: 2px solid var(--grey-500);
  }

  .sidebar.collapsed .brand {
    justify-content: center;
    padding: 0;
  }

  .brand-name {
    color: var(--grey-500);
  }

  .collapse-toggle {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 6px;
    background: none;
    border: none;
    border-radius: 8px;
    color: var(--grey-300);
    cursor: pointer;
    transition: background-color 0.15s ease, color 0.15s ease;
  }

  .collapse-toggle:hover {
    background-color: var(--grey-50);
    color: var(--grey-500);
  }

  .nav {
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding: 16px 12px;
    flex: 1;
  }

  .footer {
    padding: 16px 12px;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 10px 12px;
    border-radius: 8px;
    color: var(--grey-400);
    text-decoration: none;
    white-space: nowrap;
    transition: background-color 0.15s ease;
  }

  .sidebar.collapsed .nav-item {
    justify-content: center;
    padding: 10px 0;
  }

  .nav-item:hover {
    background-color: var(--grey-50);
  }

  .nav-item.active {
    background-color: var(--grey-50);
    color: var(--grey-500);
    font-weight: 700;
  }

  .nav-label {
    overflow: hidden;
    text-overflow: ellipsis;
  }
</style>
