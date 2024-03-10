const invoke = window.__TAURI__.core.invoke;

export async function readKeyPressesOfMonth(year, month) {
  return await invoke("read_keypresses_of_month", { year: year, month: month });
}
