# Notes

<!-- Fill this in before submission. Maximum 300 words. -->

Použil jsem DFS místo BFS, protože v této úloze nepotřebuji hledat nejkratší cestu ani vzdálenost od startovní pozice. Cílem funkce `reachable_floor_size` je pouze zjistit, kolik průchozích dlaždic je dosažitelných ze zadaného startu. Pro tento účel jsou DFS i BFS ekvivalentní: obě metody při správném použití množiny navštívených pozic projdou každou dosažitelnou dlaždici právě jednou a vrátí stejný výsledek. DFS jsem zvolil hlavně kvůli jednodušší implementaci.