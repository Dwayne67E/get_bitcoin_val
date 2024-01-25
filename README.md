# get_bitcoin_val

A noter : pour la db, il faut installer MySQL Client et MySSQLWorkbench pour voir visuellement les db avec leurs table autre part que dans le terminal. 

L'objectif de ce projet est de récupérer des valeurs grâce aux requêtes APi et d'en disposer sur une database afin d'en sortir des illustrations/graphes.

# Etape 1 : Réussir à récupérer des données grâce aux API
On a utilisé celles de Kraken et Binance, néanmoins, en y repensant on a trouvé qu'il n'était pas logique d'utiliser deux exchanges pour ce projet. 
En effet, le but est de récupérer des couples de valeurs ( exemple : le dernier prix de la paire BTC/USD tradé aisni que le volume en lot de cette dernière transaction ).
Ainsi, pour l'instant, il sera utile de ne se servir que de l'API de Kraken. 

L'étape 1 est faite. 

# Etape 2 : Réussir à lier une database à nos outputs
En effet, on arrive à récupérer le dernier prix tradé ainsi que le dernier volume tradé de manière récurrente grâce à une loop. 
L'objectif serait donc de faire migrer ces valeurs dans une table d'une database créee pour le projet. 

L'étape 2 de est à faire. 
# 07/12/2023 
L'étape 2 est maintenant faîte. 
On arrive à storer nos outputs vers une database MySQL grâce à SQLx.
 # Les nouveaux objectifs sont :
- ajouter une colonne avec la date de l'export vers la database pour pour faire un graphe en fonction du temps.
- empêcher que la table de la db se trie automatiquement selon des valeurs croissantes (c'est le ca actuellement).
- réussir à faire des graphes à partir de notre table.

# 09/12/2023
L'étape 3 est faite. On arrive à créer des graphes en allant chercher les valeurs dans la base MySQL grâce à plotters dans Rust. 
On a crée une nouvelle table avec l'id du trade (résolution des trades classés par prix par ordre croissant), le prix, le volume et le timestamp pour pour faire des graphes en fonction du temps. 

L'objectif désormais est d'essayer d'ajouter un réel intérêt ou de la pertinence. 

# 25/01/2024
Les deux projets (l'un qui récupère le prix du LTC depuis Kraken et l'envoit vers la database et celui qui réalise des graphes depuis les data de la database) ont été concaténé en un. 
Petit soucis ici,
Pour "cargo run" la partie qui vous intéresse, il faut renommer la partie voulu en main.rs (du bricolage oui mais j'ai galéré pendant 2h sans succès).
Amélioration de la partie sur les graphiques avec plotters mais il reste des soucis. 
Echec pour réaliser deux courbes sur le même graphique (l'objectif initial étant de corrélé le prix et le volume d'un actif tradé sur une plage de temps constante, nous 10sec, or là on n'arrive pas voir les deux en même temps. 
Donc la solution est de créer deux graphes séparés. Cela ne change pas grand chose mais c'est moins sympa pour la lisibilité. 

 # Objectif : rendre le code de la partie graphes plus abstraits que tout en brute. 



A noter : 
---dans le code actuel, on récupère les valeurs de la paire LTC/USD (la cryptommonnaie LiteCoin). Etrange pour un projet qui se veut get_bitcoin_value mais lorsque je remplace par XBTCZUSD il y a une erreur ( à voir ).
---il faudra aussi mieux organiser le code pour le rendre plus abstrait et plus simple pour un client. 



