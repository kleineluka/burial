<!-- Header -->
<div align="center">
    
<img src="https://github.com/kleineluka/burial/blob/main/preview/leyley.png" width="65" height="65">

# Burial 
A (pretty) one-stop-shop for [The Coffin of Andy and Leyley](https://store.steampowered.com/app/2378900/The_Coffin_of_Andy_and_Leyley/) modding 💚🩷
<br>

[Features 🐰](#features-) • [Download & Install 🩸](#install-) • [Documentation 🥩](#documentation-) • [Roadmap 👁️](#roadmap-%EF%B8%8F)

[Contribute 🥰](https://github.com/kleineluka/burial/wiki/Contributing) • [Credits 🎉](https://github.com/kleineluka/burial/blob/main/ATTRIBUTIONS.md) • [View Changelog 🍅](https://github.com/kleineluka/burial/wiki/Changelog) • [Discord 🥰](https://www.discord.gg/WWxAjJMspk)

</div>

<!-- Navigation + Preview -->
<div align="center">
    
<img src="https://github.com/kleineluka/burial/blob/main/preview/app.gif" style="width: 75%; height: auto;" alt="Loading large Burial preview.. sorry Github doesn't support webm :'( or you are previewing this markdown in VSCode, if so, thanks for working on Burial!">

</div>


<!-- Features -->
# Features 🐰
Burial is a program for TCOAAL **mod players**, **mod creators**, **content creators** and **data miners**~ 🍅

- **Mod Manager** 🍱: One-click install of modpacks, modloader manager, browse/download mods, manage installed mods, install mods from external URLs, one-click mod installation from websites (burial:// protocol), and switch mods easily with a profile system.
- **Resources** ✂️: Decrypt/encrypt .k9a files, export categories of resources, generate asset templates, parse/import dialogue, and decrypt/modify save files.
- **Reversing** 🧬: Targeted code injection, export/deobfuscate/comment game run-time code, manage NW.js SDK's, manage developer tools, view game information, and create backups of your game files.
- **Mod Tools** 🖍️:  Turn the game to an RPG Maker project, export an RPG Maker project to a mod, convert a non-Tomb mod to a Tomb mod, edit mod.json files, edit repo,json files, and generate mod version differences.
- **Knowledge** 📔: Easily access modding resources in one convenient place.
- **Easy and Safe** 🍵: Burial only takes a few clicks to get you playing mods, has lots of tooltips, and implements safety guardrails to protect your saves.
- **Quality of Life** 🌸: Pretty GUI with character-based themes, settings, built-in tutorials, update checking, lots of emojis..
- **Fast, Small, and Open** 🦄: Built with Rust, a native webview (<30mb, no Electron!), and a forever open-source mindset.

<!-- Install -->
# Install 🩸
**Executables will not be provided until a stable version is completed.**
1. **Download** the latest app from [Github Releases](https://github.com/kleineluka/burial/releases), GameBanana, Itch.io, or Nexus Mods.
2. **Run** the installer, silly!
    <br>
    Questions: [Why does Windows Defender appear?](https://github.com/kleineluka/burial/wiki/Problems-and-Questions#question-why-does-windows-defender-or-whatever-antivirus-im-using-flag-burial) and [Why is the program not portable?](https://github.com/kleineluka/burial/wiki/Problems-and-Questions#question-why-isnt-burial-portable-ex-an-exe-w-no-installer)
3. **Open** the program and **enjoy**!

To compile the source code yourself, please see the [Contributing page](https://github.com/kleineluka/burial/wiki/Contributing) on the Wiki.

<!-- Documentation -->
# Documentation 🥩
Please view the [Wiki](https://github.com/kleineluka/burial/wiki)! For help installing, go to the [Installation and Help page](https://github.com/kleineluka/burial/wiki/Installation-and-Help). For further help, check out the [Problems and Questions page](https://github.com/kleineluka/burial/wiki/Problems-and-Questions). And if all else fails? ~~Practice demonic magic with your favourite sibling-~~ Open an [issue on Github](https://github.com/kleineluka/burial/issues) or [contact me!](https://github.com/kleineluka).

Want your mod featured in Burial? Submit it to the [Llamawa.re Mod Depository](https://github.com/Llamaware/Llamaware.github.io) here! If you want your modpack featured in Burial, please fork this repo with an updated [modpacks.json](https://github.com/kleineluka/burial/blob/main/api/modpacks.json)! If you are not the creator of the mod, or your mod is not Tomb-compatible, please fork with an updated [foreign.json](https://github.com/kleineluka/burial/blob/main/api/foreign.json) (or get the mod author to submit to the depository and/or convert to Tomb!).

The tech stack is a Rust back-end (Tauri framework), a web front-end (minimal dependencies), Python scripts to aid in development, TypeScript (Deno) for code deobfuscation, and Javascript for game modification. Wanna help? Check out the [Contributing page](https://github.com/kleineluka/burial/wiki/Contributing) on the Wiki~

<!-- Roadmap -->
# Roadmap 👁️
(In no particular order and beyond basic planned features, aka luxury things that mostly™ will start after the first release..)
- [ ] Language Support
- [ ] Steam Deck / Linux Improvements
- [ ] Better Folder & File Dialog Handling 
- [ ] Easier Save Editor
- [ ] Cleanup (Code, UI, Responsiveness, Code Warnings, Var Ownership, Structure, JS -> Rust, Logging, Async I/O)
- [ ] Self Updating
- [ ] One-Click GameBanana Installation
- [ ] Symbolic Linking (for Profiles)
- [ ] Disk Space Awareness
- [ ] RAR, 7z, TAR Support (where ZIP is)
- [ ] Tauri V2 Migration
- [x] Game Instances (Profiles)
- [x] More Tooltips
- [x] Theming System
- [x] Settings System
- [x] Development Build Scripts

<!-- Footer -->
<div align="center">

<sub>**By Fans, Not Thieves 💗** A legal copy of the game is required to use Burial! Burial will **never** implement any piracy tools. </sub>
<br>
<sub>Please read the **[license](https://github.com/kleineluka/burial/blob/main/LICENSE) and [EULA](https://github.com/kleineluka/burial/blob/main/EULA.md)** before interacting with this software, and respect all licenses and rights in attributions. 🤓</sub>
<br>
<sub>This is a passion project that I work on constantly in my free-time. If you want to support me, please star 🌠 the repository or contribute!</sub>

</div>