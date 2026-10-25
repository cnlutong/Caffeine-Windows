(async () => {
  const invoke = window.__TAURI_INTERNALS__.invoke;
  const wait = async (predicate, description) => {
    for (let i = 0; i < 200; i++) {
      if (await predicate()) return;
      await new Promise(resolve => setTimeout(resolve, 25));
    }
    throw new Error(`Timeout: ${description}`);
  };
  try {
    await wait(() => document.querySelector('fieldset') && !document.querySelector('fieldset').disabled, 'React panel and event subscription');
    if (document.querySelector('h1').textContent !== 'Caffeine') throw new Error('Wrong page');
    const initial = await invoke('get_state');
    if (await invoke('smoke_reopening')) {
      if (initial.mode !== 'display') throw new Error('Closing the window lost the native mode');
      await invoke('smoke_checkpoint');
      await invoke('set_mode', { mode: 'off' });
      await invoke('smoke_checkpoint');
      await invoke('smoke_complete', { passed: true, detail: 'Real React/WebView IPC, left/right tray events, last-mode restoration, native modes checked with pmset, invalid input, localization, compact layout, window destruction/recreation with active power state, and request cleanup' });
      return;
    }
    if (initial.mode !== 'off') throw new Error('Fresh process must start off');
    // Send complete press/release pairs through the same handler as native tray events.
    const trayClick = (button, phase) => invoke('smoke_tray_click', { button, phase });
    await invoke('set_mode', { mode: 'system' });
    await invoke('set_mode', { mode: 'off' });
    await trayClick('left', 'down');
    if ((await invoke('get_state')).mode !== 'off') throw new Error('Mouse down toggled too early');
    await trayClick('left', 'up');
    if ((await invoke('get_state')).mode !== 'system') throw new Error('Left click did not enable the last mode');
    let beforeRight = await invoke('get_state');
    await trayClick('right', 'down');
    await trayClick('right', 'up');
    if ((await invoke('get_state')).revision !== beforeRight.revision) throw new Error('Right click changed the power state');
    await trayClick('left', 'down');
    await trayClick('left', 'up');
    if ((await invoke('get_state')).mode !== 'off') throw new Error('Left click did not disable wake prevention');
    beforeRight = await invoke('get_state');
    await trayClick('right', 'down');
    await trayClick('right', 'up');
    if ((await invoke('get_state')).revision !== beforeRight.revision) throw new Error('Right click enabled wake prevention');
    await invoke('set_mode', { mode: 'display' });
    for (const expected of ['off', 'display']) {
      await trayClick('left', 'down');
      await trayClick('left', 'up');
      if ((await invoke('get_state')).mode !== expected) throw new Error('Tray toggle did not restore display mode');
      await invoke('smoke_checkpoint');
    }
    await invoke('set_mode', { mode: 'off' });
    await invoke('set_language', { language: 'zh' });
    await wait(() => document.documentElement.lang === 'zh-CN', 'Chinese UI');
    for (const mode of ['system', 'display', 'system', 'off']) {
      document.querySelector(`input[value="${mode}"]`).click();
      await wait(async () => (await invoke('get_state')).mode === mode && !document.querySelector('fieldset').disabled, `Native ${mode}`);
      if (!document.querySelector(`input[value="${mode}"]`).checked) throw new Error('UI did not follow native state');
      await invoke('smoke_checkpoint');
    }
    const before = await invoke('get_state');
    await invoke('set_mode', { mode: 'invalid' }).then(() => { throw new Error('Invalid mode accepted'); }, () => {});
    const after = await invoke('get_state');
    if (after.mode !== before.mode || after.revision !== before.revision) throw new Error('Invalid input changed state');
    await invoke('set_language', { language: 'en' });
    await wait(() => document.documentElement.lang === 'en', 'English UI');
    if (document.documentElement.scrollWidth > innerWidth || document.documentElement.scrollHeight > innerHeight) throw new Error(`Compact panel overflows: viewport ${innerWidth}x${innerHeight}, content ${document.documentElement.scrollWidth}x${document.documentElement.scrollHeight}, warning=${JSON.stringify(await invoke('get_state'))}`);
    await invoke('set_mode', { mode: 'display' });
    await invoke('smoke_checkpoint');
    void invoke('smoke_reopen');
  } catch (error) {
    await invoke('set_mode', { mode: 'off' }).catch(() => {});
    await invoke('smoke_complete', { passed: false, detail: String(error) });
  }
})();
