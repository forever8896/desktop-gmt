<script lang="ts">
  import { appWindow } from "@tauri-apps/api/window";
  import { writable } from 'svelte/store';
  import { fade, slide } from 'svelte/transition';
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from 'svelte';

  const selectedTimezone = writable<string | null>(null);
  let showSettings = false;

  type UserSettings = {
    theme: {
      bg: string;
      text: string;
    };
    timezone: string | null;
    show_seconds: boolean;
  };

  let showSeconds = true;

  // Using IANA timezone identifiers for reliable DST handling
  const timezones = [
    { id: 'America/New_York', label: 'EST - New York' },
    { id: 'America/Chicago', label: 'CST - Chicago' },
    { id: 'America/Denver', label: 'MST - Denver' },
    { id: 'America/Los_Angeles', label: 'PST - Los Angeles' },
    { id: 'Europe/London', label: 'GMT - London' },
    { id: 'Europe/Paris', label: 'CET - Paris' },
    { id: 'Asia/Tokyo', label: 'JST - Tokyo' },
    { id: 'Australia/Sydney', label: 'AEST - Sydney' },
    { id: 'Pacific/Auckland', label: 'NZST - Auckland' }
  ];

  const getTime = (timezone?: string) => {
    return new Date().toLocaleTimeString('en-US', {
      timeZone: timezone,
      hour: '2-digit',
      minute: '2-digit',
      second: showSeconds ? '2-digit' : undefined,
      hour12: false
    });
  };

  let localTime = getTime();
  let targetTime = '';

  setInterval(() => {
    localTime = getTime();
    if ($selectedTimezone) {
      targetTime = getTime($selectedTimezone);
    }
  }, 1000);

  const handleClose = () => appWindow.close();

  const handleSettings = () => {
    showSettings = !showSettings;
  };

  const selectTimezone = async (timezone: string) => {
    selectedTimezone.set(timezone);
    showSettings = false;
    await saveSettings({ timezone });
  };

  let previewTime = '';
  let hoveredTimezone: string | null = null;

  const handleTimezoneHover = (timezone: string) => {
    hoveredTimezone = timezone;
    previewTime = getTime(timezone);
  };

  const handleTimezoneLeave = () => {
    hoveredTimezone = null;
    previewTime = '';
  };

  let showColorPicker = false;

  const handleColorPicker = () => {
    showColorPicker = !showColorPicker;
  };

  const colorThemes = [
    { name: 'Orange Night', bg: '#1E3E62', text: '#FF6500' },
    { name: 'Mint Dark', bg: '#2D3436', text: '#00B894' },
    { name: 'Purple Dream', bg: '#30336B', text: '#E056FD' },
    { name: 'Ocean Breeze', bg: '#182C61', text: '#48DBFB' },
    { name: 'Forest Night', bg: '#1E272E', text: '#BADC58' },
    { name: 'Rose Gold', bg: '#2F3640', text: '#FF9FF3' },
  ];

  let hoveredTheme: typeof colorThemes[0] | null = null;

  const handleThemeHover = (theme: typeof colorThemes[0]) => {
    hoveredTheme = theme;
  };

  const handleThemeLeave = () => {
    hoveredTheme = null;
  };

  const selectTheme = async (theme: typeof colorThemes[0]) => {
    try {
      await saveSettings({ 
        theme: { bg: theme.bg, text: theme.text },
        show_seconds: showSeconds
      });
      document.documentElement.style.setProperty('--bg-color', theme.bg);
      document.documentElement.style.setProperty('--text-color', theme.text);
      showColorPicker = false;
    } catch (error) {
      console.error('Failed to save theme:', error);
    }
  };

  // Create a helper function to get current theme colors
  const getCurrentTheme = () => ({
    bg: getComputedStyle(document.documentElement).getPropertyValue('--bg-color').trim(),
    text: getComputedStyle(document.documentElement).getPropertyValue('--text-color').trim(),
  });

  // Consolidate the save settings logic
  const saveSettings = async (updates: Partial<UserSettings> = {}) => {
    const currentTheme = getCurrentTheme();
    const payload = {
      bg: updates.theme?.bg ?? currentTheme.bg.replace(/\s/g, ''),
      text: updates.theme?.text ?? currentTheme.text.replace(/\s/g, ''),
      timezone: updates.timezone ?? $selectedTimezone,
      show_seconds: typeof updates.show_seconds !== 'undefined' ? updates.show_seconds : showSeconds,
      showSeconds: typeof updates.show_seconds !== 'undefined' ? updates.show_seconds : showSeconds,
    };
    
    console.log('Saving settings payload:', payload);
    try {
      await invoke('save_settings', payload);
      console.log('Settings saved successfully');
    } catch (error) {
      console.error('Failed to save settings:', error);
      throw error;
    }
  };

  // Load saved settings on startup
  const loadSavedSettings = async () => {
    try {
      const settings: UserSettings = await invoke('load_settings');
      console.log('Loaded settings:', settings);
      
      if (settings.theme) {
        const { bg, text } = settings.theme;
        document.documentElement.style.setProperty('--bg-color', bg);
        document.documentElement.style.setProperty('--text-color', text);
      }
      
      if (settings.timezone) {
        selectedTimezone.set(settings.timezone);
        targetTime = getTime(settings.timezone);
      }
      
      showSeconds = settings.show_seconds;
      
      // Force immediate update of times
      localTime = getTime();
      if ($selectedTimezone) {
        targetTime = getTime($selectedTimezone);
      }
    } catch (error) {
      console.error('Failed to load settings:', error);
      // Use default values on error
      document.documentElement.style.setProperty('--bg-color', '#1E3E62');
      document.documentElement.style.setProperty('--text-color', '#FF6500');
      showSeconds = true;
    }
  };

  // Call this when component mounts
  onMount(() => {
    loadSavedSettings();
  });

  const toggleSeconds = async () => {
    showSeconds = !showSeconds;
    // Update times immediately
    localTime = getTime();
    if ($selectedTimezone) {
      targetTime = getTime($selectedTimezone);
    }
    await saveSettings({ show_seconds: showSeconds });
  };
</script>

<svelte:head>
  <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300&display=swap" rel="stylesheet">
</svelte:head>

<div data-tauri-drag-region class="container">
  <button 
    class="settings-button" 
    on:click={handleSettings}
   
  >⚙</button>
  <button 
    class="close-button" 
    on:click={handleClose}
  >×</button>

  <div class="time-container">
    <h1>{localTime}</h1>
    <div class="time-divider"></div>
    {#if hoveredTimezone}
      <div class="preview-container">
        <h2 class="preview-time">{previewTime}</h2>
        <span class="timezone-label preview-label">{timezones.find(x => x.id === hoveredTimezone)?.label}</span>
      </div>
    {:else if $selectedTimezone}
      <div class="target-time-container">
        <h2>{targetTime}</h2>
        <span class="timezone-label">{timezones.find(x => x.id === $selectedTimezone)?.label}</span>
      </div>
    {/if}
  </div>

  {#if showSettings}
    <div transition:fade={{ duration: 200 }} class="settings-dropdown">
      {#each timezones as {id, label}}
        <button 
          class="timezone-option" 
          on:click={() => selectTimezone(id)}
          class:active={$selectedTimezone === id}
        >
          <div class="timezone-option-content">
            <span class="timezone-time" class:show-time={$selectedTimezone === id}>
              {getTime(id)}
            </span>
            <span class="timezone-label">{label}</span>
          </div>
        </button>
      {/each}
    </div>
  {/if}

  <button 
    class="palette-button" 
    on:click={handleColorPicker}
  >🎨</button>

  {#if showColorPicker}
    <div transition:fade={{ duration: 200 }} class="settings-dropdown">
      {#each colorThemes as theme}
        <button 
          class="timezone-option" 
          on:mouseenter={() => handleThemeHover(theme)}
          on:mouseleave={handleThemeLeave}
          on:click={() => selectTheme(theme)}
        >
          <div class="timezone-option-content">
            <div class="color-preview">
              <div class="color-sample" style="background-color: {theme.bg}"></div>
              <div class="color-sample" style="background-color: {theme.text}"></div>
            </div>
            <span class="timezone-label">{theme.name}</span>
          </div>
        </button>
      {/each}
    </div>
  {/if}

  <button 
    class="seconds-toggle" 
    on:click={toggleSeconds}
    title={showSeconds ? "Hide seconds" : "Show seconds"}
  >
    {showSeconds ? "s" : "s"}
  </button>
</div>

<style>
  :root {
    --bg-color: #1E3E62;
    --text-color: #FF6500;
  }

  :global(:root) {
    --bg-color: #1E3E62;
    --text-color: #FF6500;
  }

  .container {
    position: relative;
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100%;
    background-color: var(--bg-color);
    border-radius: 12px;
    -webkit-app-region: drag;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    
  }

  

  .close-button, .settings-button {
    position: absolute;
    width: 24px;
    height: 24px;
    border-radius: 50%;
    border: 1px solid var(--text-color);
    background: transparent;
    color: var(--text-color);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 14px;
    transition: all 0.2s ease;
    -webkit-app-region: no-drag;
    opacity: 0;
  }

  .close-button {
    top: 8px;
    right: 8px;
  }

  .settings-button {
    top: 8px;
    left: 8px;
    z-index: 10;
  }

  .close-button:hover, .settings-button:hover {
    opacity: 1;
    transform: scale(1.1);
  }

  .time-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
  }

  h1, h2 {
    color: var(--text-color);
    font-family: 'Inter', sans-serif;
    font-weight: 300;
    letter-spacing: 0.5px;
    margin: 0;
  }

  h1 {
    font-size: 1.8em;
    opacity: 0.95;
  }

  h2 {
    font-size: 1.8em;
    opacity: 0.8;
  }

  .timezone-label {
    color: var(--text-color);
    opacity: 0.6;
    font-size: 0.8em;
    margin: 0;
    font-family: 'Inter', sans-serif;
    font-weight: 300;
    letter-spacing: 0.5px;
  }

  .settings-dropdown {
    position: absolute;
    top: 0;
    left: 0;
   
    background-color: var(--bg-color);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    display: flex;
    flex-direction: column;
    gap: 4px;
    box-shadow: 0 8px 16px rgba(0, 0, 0, 0.2);
    -webkit-app-region: no-drag;
    
    overflow-y: auto;
    min-width: 200px;
    animation: fadeIn 0.2s ease-out;
    z-index: 5;
  }

  

  .timezone-option {
    background: transparent;
    border: none;
    color: var(--text-color);
    padding: 12px;
    text-align: center;
    cursor: pointer;
    border-radius: 8px;
    font-family: 'Inter', sans-serif;
    font-size: 0.9em;
    opacity: 1;
    width: 100%;
    transition: all 0.2s ease;
  }

  .timezone-option:hover {
    opacity: 1;
    background-color: rgba(255, 255, 255, 0.1);
    transform: scale(1.02);
  }

  .timezone-option.active {
    background-color: rgba(255, 255, 255, 0.15);
    opacity: 1;
  }

  .timezone-option-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    transition: all 0.2s ease;
  }

  .timezone-time {
    font-size: 1.2em;
    font-weight: 300;
    height: 0;
    opacity: 0;
    transition: all 0.2s ease;
  }

  .timezone-option:hover .timezone-time,
  .timezone-time.show-time {
    height: 1.5em;
    opacity: 1;
  }

  .timezone-label {
    font-size: 0.8em;
    opacity: 1;
  }

  :global(html, body) {
    margin: 0;
    padding: 0;
    height: 100%;
    -ms-overflow-style: none; 
scrollbar-width: none; 

  }

  ::-webkit-scrollbar {
display: none;
}

  .target-time-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
  }

 

  @keyframes slideIn {
    from {
      transform: translateX(20px);
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
  }

  .preview-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    animation: fadeIn 0.2s ease-out;
  }

  .preview-time, .preview-label {
    opacity: 0.7;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translate(-50%, -48%);
    }
    to {
      opacity: 1;
      transform: translate(-50%, -50%);
    }
  }

  .time-divider {
    width: 100px;
    height: 1px;
    background-color: var(--text-color);
    opacity: 1;
    margin: 8px 0;
  }

  .palette-button {
    position: absolute;
    bottom: 8px;
    right: 8px;
    width: 24px;
    height: 24px;
    border-radius: 50%;
    border: 1px solid var(--text-color);
    background: transparent;
    color: var(--text-color);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 14px;
    transition: all 0.2s ease;
    -webkit-app-region: no-drag;
    opacity: 0;
    z-index: 100;
  }

  .palette-button:hover {
    opacity: 1;
    transform: scale(1.1);
  }

  .color-preview {
    display: flex;
    gap: 4px;
    margin-bottom: 2px;
  }

  .color-sample {
    width: 12px;
    height: 12px;
    border-radius: 3px;
    border: 1px solid rgba(255, 255, 255, 0.1);
  }



  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .seconds-toggle {
    position: absolute;
    bottom: 8px;
    left: 8px;
    width: 24px;
    height: 24px;
    border-radius: 50%;
    border: 1px solid var(--text-color);
    background: transparent;
    color: var(--text-color);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 14px;
    transition: all 0.2s ease;
    -webkit-app-region: no-drag;
    opacity: 0;
    z-index: 100;
  }

  .seconds-toggle:hover {
    opacity: 1;
    transform: scale(1.1);
  }
</style>