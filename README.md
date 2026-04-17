# TP3 — Rust embarqué

## Objectif
Développer un firmware embarqué pour une carte afin d'utiliser plusieurs périphériques à l'aide de la bibliothèque Embassy.

## Architecture du code
- `src/bargraph.rs` : driver bargraph
- `src/gamepad.rs` : driver gamepad
- `src/encoder.rs` : driver encodeur avec `Qei` + accès PAC (`TIM2`) pour la position
- `src/bsp-ensea.rs` : mapping matériel dans `Board`
- `src/bargraph_task.rs` : gestions des tâches
- `examples/bargraph_example.rs` : démonstration bargraph
- `examples/gamepad_example.rs` : démonstration gamepad avec logs `defmt`
- `examples/encoder_example.rs` : démonstration encodeur avec logs `defmt`

## BSP
Structure du BSP :
- `board.bargraph_pins`
- `board.gamepad_pins`
- `board.encoder_pins`

Les exemples ont donc la même construction :
- `let board = Board::new();`
- récupérer les pins/périphériques du driver concerné
- instancier le driver
- faire une boucle de test

## Remarque sur l'encodeur
La définition de la position est faite par accès direct au timer (`TIM2`) via la PAC :
- `ARR` fixé à `10_000`
- `CNT` positionné à la valeur demandée (`set_position`)
- `reset()` remet à `0`

## Commandes utiles
### Exécution des démos
```bash
cargo run --example bargraph_example
cargo run --example gamepad_example
cargo run --example encoder_example
```

## Résultats des tests
- **Bargraph** : la barre monte/descend selon les valeurs
- **Gamepad** : les logs `defmt` changent (`true/false`) quand un bouton est pressé
- **Encoder** : la position varie selon la rotation, `pressed` passe à `true` sur appui bouton

## Problèmes ST-Link / probe-rs
À plusieurs reprises, nous avons eu des problèmes avec ST-Link et probe-rs alors que le build est ok. Solutions essayées :
- débrancher/rebrancher la carte
- tester un autre port/câble USB
- fermer les outils concurrents
- utiliser `--connect-under-reset`

On a configuré le runner pour forcer une vitesse SWD à 950 kHz pour limiter les erreurs intermittentes.
