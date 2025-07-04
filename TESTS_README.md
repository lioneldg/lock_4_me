# Tests Backend Rust - Application Tauri Lock-4-Me

Ce document décrit la suite de tests complète développée pour l'application Tauri Lock-4-Me côté backend (Rust).

## Structure des Tests

### 1. Tests Unitaires par Module

#### `src-tauri/src/read_write_settings.rs`
- **Couverture** : Gestion des paramètres (lecture/écriture JSON)
- **Tests inclus** :
  - Sérialisation/désérialisation des paramètres
  - Sauvegarde et chargement de fichiers
  - Création de répertoires imbriqués
  - Gestion des erreurs (fichiers inexistants, JSON invalide)
  - Tests des commandes Tauri `write_settings` et `read_settings`

#### `src-tauri/src/lock_screen.rs`
- **Couverture** : Verrouillage d'écran multiplateforme
- **Tests inclus** :
  - Validation des commandes pour chaque plateforme (Linux, macOS, Windows)
  - Structure et format des commandes
  - Gestion des erreurs
  - Tests spécifiques aux environnements de bureau Linux
  - Validation des arguments de commandes

#### `src-tauri/src/listen_bluetooth.rs`
- **Couverture** : Gestion Bluetooth et écoute des événements
- **Tests inclus** :
  - Gestion des erreurs Bluetooth personnalisées
  - Calculs RSSI et logique de seuil
  - Validation des UUID
  - Structure des événements JSON
  - Logique de timeout et de reconnexion
  - Tests asynchrones avec Tokio

#### `src-tauri/src/lib.rs`
- **Couverture** : Configuration principale de l'application Tauri
- **Tests inclus** :
  - Gestion des handles Bluetooth
  - Configuration des logs
  - Logique de visibilité des fenêtres
  - Configuration de la barre système
  - Gestion des événements de menu
  - Politiques d'activation (macOS)

#### `crates/bt_discover/src/lib.rs`
- **Couverture** : Crate de découverte Bluetooth
- **Tests inclus** :
  - Création et manipulation des structures `DiscoveredDevice`
  - Logique de filtrage par UUID
  - Gestion des types d'événements Bluetooth
  - Validation des valeurs RSSI
  - Gestion des erreurs d'adaptation Bluetooth
  - Tests de parsing UUID

### 2. Tests d'Intégration

#### `src-tauri/tests/integration_tests.rs`
- **Couverture** : Tests inter-modules et workflows complets
- **Tests inclus** :
  - Workflow complet des paramètres (écriture → lecture → validation)
  - Compatibilité entre modules (UUID des paramètres ↔ module Bluetooth)
  - Gestion des erreurs cross-module
  - Persistance avec caractères spéciaux
  - Cycle de vie des handles Bluetooth
  - Cohérence des messages d'erreur
  - Calculs RSSI intégrés
  - Tests asynchrones intégrés
  - Sérialisation/désérialisation complète

## Commandes d'Exécution

### Exécuter tous les tests
```bash
cd src-tauri
cargo test
```

### Exécuter des tests spécifiques par module
```bash
# Tests des paramètres
cargo test read_write_settings

# Tests du verrouillage d'écran
cargo test lock_screen

# Tests Bluetooth
cargo test listen_bluetooth

# Tests de la bibliothèque principale
cargo test lib

# Tests du crate bt_discover
cd ../crates/bt_discover
cargo test
```

### Exécuter les tests d'intégration uniquement
```bash
cd src-tauri
cargo test integration_tests
```

### Exécuter avec sortie détaillée
```bash
cargo test -- --nocapture
```

### Exécuter les tests ignorés (tests d'intégration système)
```bash
cargo test -- --ignored
```

## Dépendances de Test

Les dépendances suivantes ont été ajoutées pour les tests :

### `src-tauri/Cargo.toml`
```toml
[dev-dependencies]
tokio-test = "0.4"      # Tests asynchrones
mockall = "0.12"        # Mocking (pour tests futurs)
tempfile = "3.8"        # Fichiers temporaires pour tests
serial_test = "3.0"     # Tests séquentiels si nécessaire
```

### `crates/bt_discover/Cargo.toml`
```toml
[dev-dependencies]
tokio-test = "0.4"      # Tests asynchrones
mockall = "0.12"        # Mocking
```

## Couverture de Test

### Fonctionnalités Testées

1. **Gestion des Paramètres** ✅
   - Lecture/écriture JSON
   - Validation des données
   - Gestion des erreurs de fichiers

2. **Verrouillage d'Écran** ✅
   - Support multiplateforme
   - Validation des commandes
   - Gestion des erreurs système

3. **Bluetooth** ✅
   - Découverte d'appareils
   - Calculs RSSI
   - Gestion des événements
   - Validation UUID

4. **Configuration Tauri** ✅
   - Setup de l'application
   - Gestion des fenêtres
   - Configuration système

5. **Intégration** ✅
   - Workflows complets
   - Communication inter-modules
   - Persistance des données

### Types de Tests

- **Tests Unitaires** : ~50 tests couvrant chaque fonction/module
- **Tests d'Intégration** : ~12 tests couvrant les workflows complets
- **Tests Asynchrones** : Tests avec Tokio pour les opérations async
- **Tests Multiplateforme** : Tests spécifiques Linux/macOS/Windows
- **Tests d'Erreur** : Validation de tous les cas d'erreur

## Cas de Test Spéciaux

### Tests Ignorés par Défaut
Certains tests sont marqués `#[ignore]` car ils :
- Nécessitent des permissions système (verrouillage d'écran)
- Requièrent du matériel Bluetooth
- Peuvent interférer avec l'environnement de développement

Pour les exécuter : `cargo test -- --ignored`

### Tests Conditionnels par Plateforme
- Tests Linux : `#[cfg(target_os = "linux")]`
- Tests macOS : `#[cfg(target_os = "macos")]`
- Tests Windows : `#[cfg(target_os = "windows")]`

## Tests CI/CD

Les tests sont conçus pour fonctionner dans les environnements CI/CD :
- Pas de dépendances externes (sauf Bluetooth pour tests spécifiques)
- Utilisation de fichiers temporaires
- Gestion des erreurs appropriée pour les environnements sans interface graphique

## Amélirations Futures

1. **Mocking avancé** : Utilisation de `mockall` pour simuler les APIs Bluetooth
2. **Tests de performance** : Benchmarks pour les opérations critiques
3. **Tests de stress** : Tests de charge pour les streams Bluetooth
4. **Tests de sécurité** : Validation des entrées malveillantes

## Maintenance

- Les tests doivent être mis à jour à chaque modification de l'API
- Nouveaux tests requis pour chaque nouvelle fonctionnalité
- Revue périodique de la couverture de test avec `cargo tarpaulin`