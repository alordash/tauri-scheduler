const { invoke } = window.__TAURI__.tauri;

export async function tryLoginUser(login, password) {
    const maybeUserPromise = invoke('try_login_user', { login, password });
    return maybeUserPromise;
}
