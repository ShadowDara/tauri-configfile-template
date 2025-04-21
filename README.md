# Tauri Configfile Template

This a template for tauri 2.0 which automatically creates
a settings file for you, which will be loaded on startup
and be loaded via javascript into the frontend!

## How

Donwload the Repository then run in the root folder!

```
npm install
```

```
npm run tauri dev
```

of course you can you another package manager!

Then the Tauri App will start and create a new settings file in
*(path on windows)*

```
%appdata%\tauri-test-app\
```

The Structure for the settings file is located in

```
src-tauri\src\settings.rs
```

The settings are loaded into the frontend in `src\main.js`


## End

i hope you are enjoying this template and if some ideas,

just fork it and ad them ;)

**no credit needed for using!**
