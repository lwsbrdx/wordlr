# Changelog

## [0.3.1](https://github.com/lwsbrdx/wordlr/compare/v0.3.0...v0.3.1) (2026-06-04)


### Bug Fixes

* consolider build+homebrew dans release-please (GITHUB_TOKEN ne peut pas déclencher release.yml) ([eb8e568](https://github.com/lwsbrdx/wordlr/commit/eb8e56817122ba2407aa0a008ea38643bbece797))

## [0.3.0](https://github.com/lwsbrdx/wordlr/compare/v0.2.1...v0.3.0) (2026-06-04)


### Features

* add date above board ([90dfd8e](https://github.com/lwsbrdx/wordlr/commit/90dfd8efcf757c4c41a0de391c1d328a755398e6))
* add dictionary with seeded word of the day ([f1ef6fa](https://github.com/lwsbrdx/wordlr/commit/f1ef6fa33f0352e3ec371538d16f641fe2d27ee8))
* add menu and status bar ([8fb67a9](https://github.com/lwsbrdx/wordlr/commit/8fb67a95e96a3d144cf630ba6a1b78f52d4f5ee0))
* add shortcuts in help ([99809f4](https://github.com/lwsbrdx/wordlr/commit/99809f444f42ca1b700ead3d5bc5ce297e1d5209))
* adds quit to menu ([53db9da](https://github.com/lwsbrdx/wordlr/commit/53db9da4282a1458f33bbbe2aa0b28fdc5385eff))
* animation 3D des tiles au submit (révélation lettre par lettre, 9 steps) ([c34a131](https://github.com/lwsbrdx/wordlr/commit/c34a1317f0e89c482d11d0a5f52e58bdb734a702))
* board init and stats visibility on s char ([3175070](https://github.com/lwsbrdx/wordlr/commit/317507080b8c09b15b64d589e4a483302ba32be1))
* board state ([01033c9](https://github.com/lwsbrdx/wordlr/commit/01033c9d035dff962e56895a185c20e1c15b0f07))
* **board_state:** get non enpty lines ([f087a40](https://github.com/lwsbrdx/wordlr/commit/f087a4012030c7e0d48740d25c2353f9f6d68b34))
* change date ([370a236](https://github.com/lwsbrdx/wordlr/commit/370a236c7c609973979f7ad9082ae859cffae191))
* colors for Typed / Typing states ([d1ea107](https://github.com/lwsbrdx/wordlr/commit/d1ea107574ef68ff6d2707e53e7bf39169ef2bb2))
* get actual serie ([b8dd111](https://github.com/lwsbrdx/wordlr/commit/b8dd11143bdf90b8510b72a8ea65dd9d89b78c91))
* get best serie ([6b82ab5](https://github.com/lwsbrdx/wordlr/commit/6b82ab50a824ed99cbc11a9bf775f32c417b2fab))
* handle losses in stats popup ([93eac3b](https://github.com/lwsbrdx/wordlr/commit/93eac3b23384092488fdf726ad3e6f438bdfd3d2))
* handle submission errors (not in dictionary, highlight invalid tiles) ([c558c6e](https://github.com/lwsbrdx/wordlr/commit/c558c6efd004e5df1db5749d306a50e881b2d574))
* **help:** adds help popup ([b649533](https://github.com/lwsbrdx/wordlr/commit/b6495335c404d27b660823a35d119599dc2488c5))
* insert mode ([1d7c550](https://github.com/lwsbrdx/wordlr/commit/1d7c5509111b3eff749552964a5aa400b1f1cd94))
* **popup:** adds buy me a coffee button ([ad93456](https://github.com/lwsbrdx/wordlr/commit/ad93456cbe72c96eba8dcdd7ea90e858d62b559b))
* quitter avec q depuis n'importe quel mode ([6689385](https://github.com/lwsbrdx/wordlr/commit/66893857077b50912aafe3edf23b4223efabd4ad))
* release-please + mise à jour automatique du tap Homebrew ([7613e7a](https://github.com/lwsbrdx/wordlr/commit/7613e7a0ff879dd55e881b750871a3a341e641f4))
* rendering bottom of popup ([ea5edd9](https://github.com/lwsbrdx/wordlr/commit/ea5edd993eded35f84d74adc94e16dbf04fe4cfa))
* rendering performances ([3b15a69](https://github.com/lwsbrdx/wordlr/commit/3b15a69926675bb3abd78c50de0a5f2f8f211acd))
* submit word (validation not handled completely) ([cc06aa6](https://github.com/lwsbrdx/wordlr/commit/cc06aa6b11ad8313dd3f60223461a3dac9744c04))
* translate to french the ui ([443926a](https://github.com/lwsbrdx/wordlr/commit/443926a06886e6aa96f40da9c57b77c713d0c6fb))


### Bug Fixes

* board shouldnt tell on next line call the state of the current tile ([e624d14](https://github.com/lwsbrdx/wordlr/commit/e624d14ad7cac4d877e112abfe8a7ac865d9d8a1))
* **board:** init until last line ([56a6eb9](https://github.com/lwsbrdx/wordlr/commit/56a6eb99adec587c36bd162c4e486c0e30a60199))
* corriger le trigger update-homebrew (heredoc cassait le YAML) ([1ba8c76](https://github.com/lwsbrdx/wordlr/commit/1ba8c762ee74bd20db8ef8e34d46a75dc10ffbd9))
* créer la GameStats au démarrage et supprimer le guard redondant dans submit() ([3f71737](https://github.com/lwsbrdx/wordlr/commit/3f71737226dcbf24ba923ae2260d727489797c83))
* **dictionnary:** O(n) to O(1) ([e7f6c83](https://github.com/lwsbrdx/wordlr/commit/e7f6c83af75f39e4604f82bdea1d3fde18b5a754))
* éviter le double appel à dirs::data_dir() ([3635a3d](https://github.com/lwsbrdx/wordlr/commit/3635a3ddd0c14f67eb7b0019153e694f13f0b666))
* extraire Color::Rgb(205, 135, 41) en constante COLOR_PRESENT ([5bf2c6f](https://github.com/lwsbrdx/wordlr/commit/5bf2c6fdba3d47ba64667b1854ce1694487fa60f))
* gérer SubmissionError::TooShort avec highlight_empty_tiles ([9f3aee6](https://github.com/lwsbrdx/wordlr/commit/9f3aee678cc309f648d2717bd109a0432fa2bd8c))
* larger status bar helper Constraint ([19b692a](https://github.com/lwsbrdx/wordlr/commit/19b692af2b2b2b2683c3f233284a18583a505518))
* last tile of current line stay in typing mode ([cfc3d70](https://github.com/lwsbrdx/wordlr/commit/cfc3d7029c5f78f992aaf0cc13643b8717979d60))
* layout et style ([b1f08cb](https://github.com/lwsbrdx/wordlr/commit/b1f08cb4dbf470595943fcfb60ce8d2819b2ccb1))
* masquer le mot secret dans la popup si la partie n'est pas terminée ([93ea896](https://github.com/lwsbrdx/wordlr/commit/93ea8964f5c6e5dda6f7f59f7de5c08cefd56b0a))
* popup ui fixes ([0ddf622](https://github.com/lwsbrdx/wordlr/commit/0ddf6226112170923188f5859cca9379dc01b8f3))
* **popup:** popup uses games_stats reference ([a749cc8](https://github.com/lwsbrdx/wordlr/commit/a749cc82fee6a65c21126a77361423d01181db3e))
* **popup:** ui fixes ([561710b](https://github.com/lwsbrdx/wordlr/commit/561710b57715e2b084645386f5698c3a43d9b0eb))
* series calculations ([fe2edf0](https://github.com/lwsbrdx/wordlr/commit/fe2edf026616e1993988549000d2062c54b03f67))
* tags release-please en v* simple + trigger release.yml sur wordlr-v* aussi ([4b56e6e](https://github.com/lwsbrdx/wordlr/commit/4b56e6e6573cd7e775e67192a4c78cf9b0ff8ef2))
* unused field ([ffd12d8](https://github.com/lwsbrdx/wordlr/commit/ffd12d85e9688147a220a4f7c8654e6594a52217))
* **validator:** 2 passes algorithm ([c099b71](https://github.com/lwsbrdx/wordlr/commit/c099b713c99babf3598e9aa59769dbd024717d6d))


### Performance Improvements

* utiliser un HashSet statique pour Dictionnary::contains() en O(1) ([cb78636](https://github.com/lwsbrdx/wordlr/commit/cb78636ba41180e3d9e9811559e2ed63b7c70ae2))

## [0.2.1](https://github.com/lwsbrdx/wordlr/compare/wordlr-v0.2.0...wordlr-v0.2.1) (2026-06-04)


### Bug Fixes

* corriger le trigger update-homebrew (heredoc cassait le YAML) ([1ba8c76](https://github.com/lwsbrdx/wordlr/commit/1ba8c762ee74bd20db8ef8e34d46a75dc10ffbd9))

## [0.2.0](https://github.com/lwsbrdx/wordlr/compare/wordlr-v0.1.0...wordlr-v0.2.0) (2026-06-04)


### Features

* add date above board ([90dfd8e](https://github.com/lwsbrdx/wordlr/commit/90dfd8efcf757c4c41a0de391c1d328a755398e6))
* add dictionary with seeded word of the day ([f1ef6fa](https://github.com/lwsbrdx/wordlr/commit/f1ef6fa33f0352e3ec371538d16f641fe2d27ee8))
* add menu and status bar ([8fb67a9](https://github.com/lwsbrdx/wordlr/commit/8fb67a95e96a3d144cf630ba6a1b78f52d4f5ee0))
* add shortcuts in help ([99809f4](https://github.com/lwsbrdx/wordlr/commit/99809f444f42ca1b700ead3d5bc5ce297e1d5209))
* adds quit to menu ([53db9da](https://github.com/lwsbrdx/wordlr/commit/53db9da4282a1458f33bbbe2aa0b28fdc5385eff))
* animation 3D des tiles au submit (révélation lettre par lettre, 9 steps) ([c34a131](https://github.com/lwsbrdx/wordlr/commit/c34a1317f0e89c482d11d0a5f52e58bdb734a702))
* board init and stats visibility on s char ([3175070](https://github.com/lwsbrdx/wordlr/commit/317507080b8c09b15b64d589e4a483302ba32be1))
* board state ([01033c9](https://github.com/lwsbrdx/wordlr/commit/01033c9d035dff962e56895a185c20e1c15b0f07))
* **board_state:** get non enpty lines ([f087a40](https://github.com/lwsbrdx/wordlr/commit/f087a4012030c7e0d48740d25c2353f9f6d68b34))
* change date ([370a236](https://github.com/lwsbrdx/wordlr/commit/370a236c7c609973979f7ad9082ae859cffae191))
* colors for Typed / Typing states ([d1ea107](https://github.com/lwsbrdx/wordlr/commit/d1ea107574ef68ff6d2707e53e7bf39169ef2bb2))
* get actual serie ([b8dd111](https://github.com/lwsbrdx/wordlr/commit/b8dd11143bdf90b8510b72a8ea65dd9d89b78c91))
* get best serie ([6b82ab5](https://github.com/lwsbrdx/wordlr/commit/6b82ab50a824ed99cbc11a9bf775f32c417b2fab))
* handle losses in stats popup ([93eac3b](https://github.com/lwsbrdx/wordlr/commit/93eac3b23384092488fdf726ad3e6f438bdfd3d2))
* handle submission errors (not in dictionary, highlight invalid tiles) ([c558c6e](https://github.com/lwsbrdx/wordlr/commit/c558c6efd004e5df1db5749d306a50e881b2d574))
* **help:** adds help popup ([b649533](https://github.com/lwsbrdx/wordlr/commit/b6495335c404d27b660823a35d119599dc2488c5))
* insert mode ([1d7c550](https://github.com/lwsbrdx/wordlr/commit/1d7c5509111b3eff749552964a5aa400b1f1cd94))
* **popup:** adds buy me a coffee button ([ad93456](https://github.com/lwsbrdx/wordlr/commit/ad93456cbe72c96eba8dcdd7ea90e858d62b559b))
* quitter avec q depuis n'importe quel mode ([6689385](https://github.com/lwsbrdx/wordlr/commit/66893857077b50912aafe3edf23b4223efabd4ad))
* release-please + mise à jour automatique du tap Homebrew ([7613e7a](https://github.com/lwsbrdx/wordlr/commit/7613e7a0ff879dd55e881b750871a3a341e641f4))
* rendering bottom of popup ([ea5edd9](https://github.com/lwsbrdx/wordlr/commit/ea5edd993eded35f84d74adc94e16dbf04fe4cfa))
* rendering performances ([3b15a69](https://github.com/lwsbrdx/wordlr/commit/3b15a69926675bb3abd78c50de0a5f2f8f211acd))
* submit word (validation not handled completely) ([cc06aa6](https://github.com/lwsbrdx/wordlr/commit/cc06aa6b11ad8313dd3f60223461a3dac9744c04))
* translate to french the ui ([443926a](https://github.com/lwsbrdx/wordlr/commit/443926a06886e6aa96f40da9c57b77c713d0c6fb))


### Bug Fixes

* board shouldnt tell on next line call the state of the current tile ([e624d14](https://github.com/lwsbrdx/wordlr/commit/e624d14ad7cac4d877e112abfe8a7ac865d9d8a1))
* **board:** init until last line ([56a6eb9](https://github.com/lwsbrdx/wordlr/commit/56a6eb99adec587c36bd162c4e486c0e30a60199))
* créer la GameStats au démarrage et supprimer le guard redondant dans submit() ([3f71737](https://github.com/lwsbrdx/wordlr/commit/3f71737226dcbf24ba923ae2260d727489797c83))
* **dictionnary:** O(n) to O(1) ([e7f6c83](https://github.com/lwsbrdx/wordlr/commit/e7f6c83af75f39e4604f82bdea1d3fde18b5a754))
* éviter le double appel à dirs::data_dir() ([3635a3d](https://github.com/lwsbrdx/wordlr/commit/3635a3ddd0c14f67eb7b0019153e694f13f0b666))
* extraire Color::Rgb(205, 135, 41) en constante COLOR_PRESENT ([5bf2c6f](https://github.com/lwsbrdx/wordlr/commit/5bf2c6fdba3d47ba64667b1854ce1694487fa60f))
* gérer SubmissionError::TooShort avec highlight_empty_tiles ([9f3aee6](https://github.com/lwsbrdx/wordlr/commit/9f3aee678cc309f648d2717bd109a0432fa2bd8c))
* larger status bar helper Constraint ([19b692a](https://github.com/lwsbrdx/wordlr/commit/19b692af2b2b2b2683c3f233284a18583a505518))
* last tile of current line stay in typing mode ([cfc3d70](https://github.com/lwsbrdx/wordlr/commit/cfc3d7029c5f78f992aaf0cc13643b8717979d60))
* layout et style ([b1f08cb](https://github.com/lwsbrdx/wordlr/commit/b1f08cb4dbf470595943fcfb60ce8d2819b2ccb1))
* masquer le mot secret dans la popup si la partie n'est pas terminée ([93ea896](https://github.com/lwsbrdx/wordlr/commit/93ea8964f5c6e5dda6f7f59f7de5c08cefd56b0a))
* popup ui fixes ([0ddf622](https://github.com/lwsbrdx/wordlr/commit/0ddf6226112170923188f5859cca9379dc01b8f3))
* **popup:** popup uses games_stats reference ([a749cc8](https://github.com/lwsbrdx/wordlr/commit/a749cc82fee6a65c21126a77361423d01181db3e))
* **popup:** ui fixes ([561710b](https://github.com/lwsbrdx/wordlr/commit/561710b57715e2b084645386f5698c3a43d9b0eb))
* series calculations ([fe2edf0](https://github.com/lwsbrdx/wordlr/commit/fe2edf026616e1993988549000d2062c54b03f67))
* unused field ([ffd12d8](https://github.com/lwsbrdx/wordlr/commit/ffd12d85e9688147a220a4f7c8654e6594a52217))
* **validator:** 2 passes algorithm ([c099b71](https://github.com/lwsbrdx/wordlr/commit/c099b713c99babf3598e9aa59769dbd024717d6d))


### Performance Improvements

* utiliser un HashSet statique pour Dictionnary::contains() en O(1) ([cb78636](https://github.com/lwsbrdx/wordlr/commit/cb78636ba41180e3d9e9811559e2ed63b7c70ae2))
