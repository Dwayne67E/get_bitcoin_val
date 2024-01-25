# get_bitcoin_val

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



A noter : 
---dans le code actuel, on récupère les valeurs de la paire LTC/USD (la cryptommonnaie LiteCoin). Etrange pour un projet qui se veut get_bitcoin_value mais lorsque je remplace par XBTCZUSD il y a une erreur ( à voir ).
---il faudra aussi mieux organiser le code pour le rendre plus abstrait et plus simple pour un client. 



