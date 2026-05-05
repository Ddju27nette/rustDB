# rustdb - Système de Gestion de Base de Données en Rust

Ce projet est réalisé dans le cadre du **TP de Rust** par le **Groupe 15**. Il consiste en l'implémentation des composants fondamentaux d'un moteur de base de données relationnelle en utilisant le langage Rust pour garantir la sécurité mémoire et la performance.

##  Fonctionnalités implémentées

Le projet est découpé en plusieurs modules gérés via des branches spécifiques :

*   **Gestion du Disque (`src/storage`)** : Couche d'abstraction pour la lecture et l'écriture des données sur le support physique.
*   **Journalisation (`src/wal`)** : Implémentation du journal d'écriture anticipée (Write-Ahead Logging - WAL) pour la persistance et la reprise sur incident.
*   **Gestion des Transactions (`src/tx`)** : Support des propriétés ACID pour garantir l'intégrité des données lors d'opérations concurrentes.
*   **Indexation (`src/btree`)** : Structure de données (B-Tree ou Hash) permettant d'accélérer la recherche des enregistrements.

##  Architecture du Projet

```text
rustdb/
├── src/
│   ├── storage/      # Gestion du stockage physique
│   ├── wal/      # Système de journalisation (WAL)
│   ├── tx/           # Gestionnaire de transactions
│   ├── btree/        # Moteur d'indexation
│   └── main.rs       # Point d'entrée de l'application
│   └── lib.rs       # API
├── target/           # (Ignoré par Git) Fichiers de compilation
├── Cargo.toml        # Dépendances et métadonnées du projet
└── .gitignore        # Configuration des fichiers à ignorer
