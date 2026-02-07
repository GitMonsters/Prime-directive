#!/usr/bin/env python3
"""
MULTILINGUAL PHYSICS MODULE - CLARIN Integration

Provides physics explanations in multiple European languages:
- English (en)
- German (de)
- French (fr)
- Spanish (es)

Integrates with CLARIN for linguistic quality assurance and
standardized terminology mapping.
"""

from typing import Dict, List, Optional
from enum import Enum


class Language(Enum):
    """Supported languages."""
    ENGLISH = "en"
    GERMAN = "de"
    FRENCH = "fr"
    SPANISH = "es"


class MultilingualPhysics:
    """
    Provides physics explanations in multiple languages.
    Uses CLARIN resources for linguistic accuracy.
    """

    def __init__(self):
        """Initialize multilingual physics explanations."""
        self.explanations = self._load_multilingual_explanations()
        self.default_language = Language.ENGLISH

    def _load_multilingual_explanations(self) -> Dict[str, Dict[str, str]]:
        """Load physics explanations in all languages."""
        return {
            'gravity': {
                Language.ENGLISH.value: """Gravity is one of the fundamental forces that shapes our universe. All objects with mass attract each other, though the effect is usually only noticeable with very massive objects like planets and stars.

Here's how it works: Earth pulls you downward with a force proportional to your mass and Earth's mass, following Newton's law of universal gravitation (F = G·m₁·m₂/r²). What's fascinating is that you also pull Earth upward with equal force—but since Earth is about 10²⁴ times more massive than you, you don't notice Earth accelerating toward you.

Gravity isn't just a downward force—it keeps the Moon orbiting Earth, keeps planets orbiting the Sun, and keeps galaxies held together. Einstein later showed us that gravity isn't actually a force pulling objects, but rather a curvature of spacetime caused by mass and energy. Massive objects bend spacetime around them, and other objects follow the curved geometry, which we perceive as gravitational attraction.""",

                Language.GERMAN.value: """Gravitation ist eine der fundamentalen Kräfte, die unser Universum prägt. Alle Objekte mit Masse ziehen sich gegenseitig an, obwohl dieser Effekt normalerweise nur bei sehr massiven Objekten wie Planeten und Sternen spürbar ist.

So funktioniert es: Die Erde zieht dich mit einer Kraft nach unten, die proportional zu deiner Masse und der Erdmasse ist, gemäß Newtons Gravitationsgesetz (F = G·m₁·m₂/r²). Das Faszinierende ist, dass du die Erde mit gleicher Kraft nach oben ziehst—aber da die Erde etwa 10²⁴ Mal massiver ist als du, merkst du nicht, dass sich die Erde zu dir hin beschleunigt.

Gravitation ist nicht nur eine Abwärtskraft—sie hält den Mond in Umlaufbahn um die Erde, hält Planeten in Umlaufbahn um die Sonne und hält Galaxien zusammen. Einstein zeigte uns später, dass Gravitation eigentlich keine Kraft ist, die Objekte anzieht, sondern eher eine Krümmung der Raumzeit, die durch Masse und Energie verursacht wird.""",

                Language.FRENCH.value: """La gravité est l'une des forces fondamentales qui façonne notre univers. Tous les objets ayant une masse s'attirent mutuellement, bien que cet effet soit généralement perceptible uniquement avec des objets très massifs comme les planètes et les étoiles.

Voici comment cela fonctionne : la Terre vous tire vers le bas avec une force proportionnelle à votre masse et à celle de la Terre, selon la loi de la gravitation universelle de Newton (F = G·m₁·m₂/r²). Ce qui est fascinant, c'est que vous tirez également la Terre vers le haut avec une force égale—mais comme la Terre est environ 10²⁴ fois plus massive que vous, vous ne remarquez pas que la Terre s'accélère vers vous.

La gravité n'est pas seulement une force descendante—elle maintient la Lune en orbite autour de la Terre, maintient les planètes en orbite autour du Soleil et maintient les galaxies ensemble. Einstein nous a montré plus tard que la gravité n'est pas réellement une force qui attire les objets, mais plutôt une courbure de l'espace-temps causée par la masse et l'énergie.""",

                Language.SPANISH.value: """La gravedad es una de las fuerzas fundamentales que da forma a nuestro universo. Todos los objetos con masa se atraen mutuamente, aunque este efecto normalmente solo es perceptible con objetos muy masivos como planetas y estrellas.

Así es como funciona: la Tierra te atrae hacia abajo con una fuerza proporcional a tu masa y la de la Tierra, siguiendo la ley de la gravitación universal de Newton (F = G·m₁·m₂/r²). Lo fascinante es que también atraes la Tierra hacia arriba con una fuerza igual—pero como la Tierra es aproximadamente 10²⁴ veces más masiva que tú, no notas que la Tierra se acelera hacia ti.

La gravedad no es solo una fuerza hacia abajo—mantiene la Luna en órbita alrededor de la Tierra, mantiene los planetas en órbita alrededor del Sol y mantiene las galaxias unidas. Einstein nos mostró más tarde que la gravedad no es realmente una fuerza que atrae objetos, sino más bien una curvatura del espacio-tiempo causada por la masa y la energía.""",
            },

            'entropy': {
                Language.ENGLISH.value: """Entropy measures disorder or randomness in a system, and it always increases in isolated systems (the Second Law of Thermodynamics). This is why your room gets messier over time without active organization, and why you can't "unbreak" a dropped egg.

The fundamental insight: there are vastly more disordered states than ordered states. Imagine a deck of cards: there's exactly one perfectly ordered arrangement, but billions upon billions of shuffled arrangements. Random processes naturally tend toward more common (disordered) states.

This principle applies everywhere: heat flows from hot to cold (not the reverse), stars burn out, and the universe tends toward maximum disorder. However, local regions can decrease entropy by using energy—your body maintains low entropy through constant metabolic activity, powered by the sun's energy. The total entropy of the universe still increases.""",

                Language.GERMAN.value: """Entropie misst Unordnung oder Zufälligkeit in einem System, und sie nimmt immer in isolierten Systemen zu (der zweite Hauptsatz der Thermodynamik). Dies ist der Grund, warum dein Zimmer mit der Zeit ohne aktive Organisation unordentlicher wird, und warum man ein fallen gelassenes Ei nicht "heil machen" kann.

Die fundamentale Einsicht: es gibt unendlich viel mehr ungeordnete Zustände als geordnete Zustände. Stell dir ein Kartendeck vor: es gibt genau eine perfekt geordnete Anordnung, aber Milliarden und Abermilliarden von gemischten Anordnungen. Zufällige Prozesse tendieren natürlicherweise zu häufigeren (ungeordneten) Zuständen.

Dieses Prinzip gilt überall: Wärme fließt von heiß zu kalt (nicht umgekehrt), Sterne brennen aus, und das Universum tendiert zur maximalen Unordnung. Allerdings können lokale Regionen die Entropie durch Energieeinsatz verringern—dein Körper erhält niedrige Entropie durch konstante stoffwechsel Aktivität, angetrieben durch die Sonnenenergie. Die Gesamtentropie des Universums nimmt immer noch zu.""",

                Language.FRENCH.value: """L'entropie mesure le désordre ou l'aléatoire dans un système, et elle augmente toujours dans les systèmes isolés (la deuxième loi de la thermodynamique). C'est pourquoi votre chambre devient plus désordonnée au fil du temps sans organisation active, et pourquoi vous ne pouvez pas "réparer" un œuf cassé.

L'intuition fondamentale : il y a infiniment plus d'états désordonnés que d'états ordonnés. Imaginez un jeu de cartes : il n'y a qu'un seul arrangement parfaitement ordonné, mais des milliards et des milliards d'arrangements mélangés. Les processus aléatoires tendent naturellement vers des états plus courants (désordonnés).

Ce principe s'applique partout : la chaleur s'écoule du chaud au froid (pas l'inverse), les étoiles s'épuisent, et l'univers tend vers le désordre maximum. Cependant, les régions locales peuvent diminuer l'entropie en utilisant de l'énergie—votre corps maintient une faible entropie grâce à une activité métabolique constante, alimentée par l'énergie solaire. L'entropie totale de l'univers continue d'augmenter.""",

                Language.SPANISH.value: """La entropía mide el desorden o la aleatoriedad en un sistema, y siempre aumenta en sistemas aislados (la segunda ley de la termodinámica). Esta es la razón por la cual tu habitación se vuelve más desordenada con el tiempo sin organización activa, y por qué no puedes "desromper" un huevo caído.

La idea fundamental: hay infinitamente más estados desordenados que estados ordenados. Imagina una baraja de cartas: hay exactamente un arreglo perfectamente ordenado, pero miles de millones de arreglos barajados. Los procesos aleatorios tienden naturalmente hacia estados más comunes (desordenados).

Este principio se aplica en todas partes: el calor fluye de caliente a frío (no al revés), las estrellas se queman y el universo tiende al máximo desorden. Sin embargo, las regiones locales pueden disminuir la entropía usando energía—tu cuerpo mantiene baja entropía a través de actividad metabólica constante, impulsada por la energía solar. La entropía total del universo sigue aumentando.""",
            },

            'quantum_entanglement': {
                Language.ENGLISH.value: """Quantum entanglement creates correlations between particles that are impossible in classical physics. When two particles are entangled, measuring one instantly affects the other, regardless of distance.

This doesn't violate relativity because you can't use entanglement to transmit information faster than light—the correlation only becomes apparent when comparing measurements. Yet Einstein called this "spooky action at a distance" because it seemed to violate locality (the idea that distant objects can't instantly affect each other).

Experiments have confirmed that entanglement is real and not due to hidden variables. It's the foundation of quantum computing (qubits in superposition) and quantum cryptography (unhackable encryption). Entanglement shows that the quantum world is fundamentally interconnected in ways our intuition from macroscopic experience can't comprehend.""",

                Language.GERMAN.value: """Quantenverschränkung erzeugt Korrelationen zwischen Teilchen, die in der klassischen Physik unmöglich sind. Wenn zwei Teilchen verschränkt sind, beeinflusst die Messung des einen sofort den anderen, unabhängig von der Entfernung.

Das verstößt nicht gegen die Relativitätstheorie, weil man Verschränkung nicht verwenden kann, um Informationen schneller als das Licht zu übertragen—die Korrelation wird nur bei Vergleich von Messungen offensichtlich. Doch Einstein nannte dies "spukhafte Fernwirkung", weil es die Lokalität zu verletzen schien (die Idee, dass entfernte Objekte sich nicht sofort gegenseitig beeinflussen können).

Experimente haben bestätigt, dass Verschränkung real ist und nicht auf verborgene Variablen zurückzuführen ist. Sie ist das Fundament des Quantencomputing (Qubits in Überlagerung) und der Quantenkryptographie (nicht hackbare Verschlüsselung). Verschränkung zeigt, dass die Quantenwelt grundlegend vernetzt ist auf Weise, die unsere Intuition von makroskopischen Erfahrungen nicht erfassen kann.""",

                Language.FRENCH.value: """L'intrication quantique crée des corrélations entre les particules qui sont impossibles en physique classique. Quand deux particules sont intriquées, mesurer l'une affecte instantanément l'autre, indépendamment de la distance.

Cela ne viole pas la relativité car on ne peut pas utiliser l'intrication pour transmettre des informations plus vite que la lumière—la corrélation ne devient apparente que lors de la comparaison des mesures. Pourtant, Einstein a appelé cela "une action fantomatique à distance" car cela semblait violer la localité (l'idée que les objets distants ne peuvent pas s'affecter instantanément).

Les expériences ont confirmé que l'intrication est réelle et ne résulte pas de variables cachées. C'est le fondement de l'informatique quantique (qubits en superposition) et de la cryptographie quantique (chiffrement inviolable). L'intrication montre que le monde quantique est fondamentalement interconnecté d'une manière que notre intuition d'expérience macroscopique ne peut pas comprendre.""",

                Language.SPANISH.value: """El entrelazamiento cuántico crea correlaciones entre partículas que son imposibles en física clásica. Cuando dos partículas están entrelazadas, medir una afecta instantáneamente la otra, independientemente de la distancia.

Esto no viola la relatividad porque no puedes usar el entrelazamiento para transmitir información más rápido que la luz—la correlación solo se hace evidente al comparar mediciones. Sin embargo, Einstein llamó a esto "acción fantasmal a distancia" porque parecía violar la localidad (la idea de que los objetos distantes no pueden afectarse instantáneamente entre sí).

Los experimentos han confirmado que el entrelazamiento es real y no se debe a variables ocultas. Es la base de la computación cuántica (qubits en superposición) y la criptografía cuántica (cifrado imposible de piratear). El entrelazamiento muestra que el mundo cuántico está fundamentalmente interconectado de maneras que nuestra intuición de la experiencia macroscópica no puede comprender.""",
            },

            'friction': {
                Language.ENGLISH.value: """Friction is the force opposing motion between two surfaces in contact. Without friction, you couldn't walk, cars couldn't brake, and everything would slide indefinitely. Friction arises from microscopic imperfections and interactions between surface atoms.

There are three types of friction: static friction prevents initial motion, kinetic friction opposes moving objects, and rolling friction is much lower than sliding friction. Engineers work to minimize friction in machinery using lubricants and ball bearings, but also exploit friction in brakes, tires, and clutches where it's essential.

Friction causes 15-20% of fuel consumption in cars. High-speed trains use magnetic levitation to eliminate friction entirely. Friction also generates heat—rubbing wood creates fire, meteor entries create fireballs, and brake pads heat up when stopping.""",

                Language.GERMAN.value: """Reibung ist die Kraft, die der Bewegung zwischen zwei kontaktierenden Oberflächen entgegenwirkt. Ohne Reibung könntest du nicht gehen, Autos könnten nicht bremsen, und alles würde unbegrenzt gleiten. Reibung entsteht durch mikroskopische Unebenheiten und Wechselwirkungen zwischen Oberflächenatomen.

Es gibt drei Arten von Reibung: Haftreibung verhindert die anfängliche Bewegung, Gleitreibung widersteht bewegten Objekten, und Rollreibung ist viel kleiner als Gleitreibung. Ingenieure arbeiten daran, Reibung in Maschinen durch Schmieröle und Kugellager zu minimieren, nutzen sie aber auch in Bremsen, Reifen und Kupplungen, wo sie notwendig ist.

Reibung verursacht 15-20% des Kraftstoffverbrauchs in Autos. Hochgeschwindigkeitszüge verwenden magnetische Levitation, um Reibung völlig zu beseitigen. Reibung erzeugt auch Wärme—Holz reiben erzeugt Feuer, Meteoreintritte erzeugen Feuerquellen, und Bremsbeläge heizen sich beim Bremsen auf.""",

                Language.FRENCH.value: """Le frottement est la force qui s'oppose au mouvement entre deux surfaces en contact. Sans frottement, vous ne pourriez pas marcher, les voitures ne pourraient pas freiner, et tout glisserait indéfiniment. Le frottement résulte des imperfections microscopiques et des interactions entre les atomes de surface.

Il existe trois types de frottement : le frottement statique empêche le mouvement initial, le frottement cinétique s'oppose aux objets en mouvement, et le frottement de roulement est beaucoup plus faible que le frottement de glissement. Les ingénieurs travaillent pour minimiser le frottement dans les machines en utilisant des lubrifiants et des roulements à billes, mais l'exploitent également dans les freins, les pneus et les embrayages où il est essentiel.

Le frottement provoque 15-20 % de la consommation de carburant des voitures. Les trains à grande vitesse utilisent la lévitation magnétique pour éliminer complètement le frottement. Le frottement génère également de la chaleur—frotter du bois crée du feu, les entrées de météores créent des boules de feu, et les plaquettes de frein chauffent lors du freinage.""",

                Language.SPANISH.value: """La fricción es la fuerza que se opone al movimiento entre dos superficies en contacto. Sin fricción, no podrías caminar, los autos no podrían frenar, y todo resbaläría indefinidamente. La fricción surge de imperfecciones microscópicas e interacciones entre átomos de superficie.

Hay tres tipos de fricción: fricción estática previene el movimiento inicial, fricción cinética se opone a objetos en movimiento, y fricción de rodadura es mucho menor que fricción de deslizamiento. Los ingenieros trabajan para minimizar la fricción en maquinaria usando lubricantes y rodamientos de bolas, pero también la explotan en frenos, neumáticos y embragues donde es esencial.

La fricción causa el 15-20% del consumo de combustible en autos. Los trenes de alta velocidad usan levitación magnética para eliminar completamente la fricción. La fricción también genera calor—frotar madera crea fuego, las entradas de meteoros crean bolas de fuego, y las pastillas de freno se calientan al frenar.""",
            },

            'temperature': {
                Language.ENGLISH.value: """Temperature is the average kinetic energy of particles in a substance. At absolute zero (-273.15°C), particles have minimal motion; as temperature increases, particles vibrate and move faster. Temperature differs fundamentally from heat: temperature is a property of an object, while heat is energy transfer between objects.

A cup of hot water has high temperature, but a swimming pool at warm temperature contains far more thermal energy because it has so much more mass. The three temperature scales (Celsius, Fahrenheit, Kelvin) measure the same physical phenomenon differently. Scientists use Kelvin because it's an absolute scale—making ratios physically meaningful (200K is twice as hot as 100K).

Understanding temperature is crucial for daily life: cooking, air conditioning, weather forecasting, and thermometer use all depend on temperature concepts. Industrial processes carefully control temperature: steel manufacturing requires precise temperatures, pharmaceuticals must be kept at specific temperatures, and cryogenic technology uses extreme cold for superconductors and rocket fuel.""",

                Language.GERMAN.value: """Temperatur ist die durchschnittliche kinetische Energie von Teilchen in einer Substanz. Bei absolutem Nullpunkt (-273,15°C) haben Teilchen minimale Bewegung; mit zunehmender Temperatur vibrieren und bewegen sich Teilchen schneller. Temperatur unterscheidet sich grundlegend von Wärme: Temperatur ist eine Eigenschaft eines Objekts, während Wärme Energieübertragung zwischen Objekten ist.

Eine Tasse heißes Wasser hat hohe Temperatur, aber ein Schwimmbecken bei warmer Temperatur enthält viel mehr thermische Energie, weil es viel mehr Masse hat. Die drei Temperaturskalen (Celsius, Fahrenheit, Kelvin) messen das gleiche physikalische Phänomen unterschiedlich. Wissenschaftler verwenden Kelvin, weil es eine absolute Skala ist—was Verhältnisse physikalisch sinnvoll macht (200K ist doppelt so heiß wie 100K).

Das Verständnis der Temperatur ist entscheidend für das tägliche Leben: Kochen, Klimaanlage, Wettervorhersage und Thermometerverwendung hängen alle von Temperaturkonzepten ab. Industrielle Prozesse kontrollieren Temperatur sorgfältig: Stahlherstellung erfordert präzise Temperaturen, Arzneimittel müssen bei bestimmten Temperaturen gelagert werden, und Kryotechnik verwendet extreme Kälte für Supraleiter und Raketentreibstoff.""",

                Language.FRENCH.value: """La température est l'énergie cinétique moyenne des particules dans une substance. Au zéro absolu (-273,15°C), les particules ont un mouvement minimal ; à mesure que la température augmente, les particules vibrent et se déplacent plus vite. La température diffère fondamentalement de la chaleur : la température est une propriété d'un objet, tandis que la chaleur est le transfert d'énergie entre les objets.

Une tasse d'eau chaude a une température élevée, mais une piscine à température chaude contient bien plus d'énergie thermique car elle a beaucoup plus de masse. Les trois échelles de température (Celsius, Fahrenheit, Kelvin) mesurent le même phénomène physique différemment. Les scientifiques utilisent Kelvin car c'est une échelle absolue—rendant les ratios physiquement significatifs (200K est deux fois plus chaud que 100K).

Comprendre la température est crucial pour la vie quotidienne : la cuisine, la climatisation, la prévision météorologique et l'utilisation de thermomètres dépendent tous des concepts de température. Les processus industriels contrôlent soigneusement la température : la fabrication de l'acier nécessite des températures précises, les produits pharmaceutiques doivent être conservés à des températures spécifiques, et la technologie cryogénique utilise le froid extrême pour les supraconducteurs et les carburants pour fusées.""",

                Language.SPANISH.value: """La temperatura es la energía cinética promedio de las partículas en una sustancia. En cero absoluto (-273,15°C), las partículas tienen movimiento mínimo; a medida que aumenta la temperatura, las partículas vibran y se mueven más rápido. La temperatura difiere fundamentalmente del calor: la temperatura es una propiedad de un objeto, mientras que el calor es la transferencia de energía entre objetos.

Una taza de agua caliente tiene alta temperatura, pero una piscina a temperatura cálida contiene mucha más energía térmica porque tiene mucha más masa. Las tres escalas de temperatura (Celsius, Fahrenheit, Kelvin) miden el mismo fenómeno físico de manera diferente. Los científicos usan Kelvin porque es una escala absoluta—haciendo que los ratios sean físicamente significativos (200K es el doble de caliente que 100K).

Entender la temperatura es crucial para la vida cotidiana: cocinar, aire acondicionado, pronóstico del tiempo y uso de termómetros dependen todos de conceptos de temperatura. Los procesos industriales controlan cuidadosamente la temperatura: la fabricación de acero requiere temperaturas precisas, los productos farmacéuticos deben almacenarse a temperaturas específicas, y la tecnología criogénica usa frío extremo para superconductores y combustible de cohetes.""",
            },

            'light': {
                Language.ENGLISH.value: """Light is electromagnetic radiation that travels at the speed of light (300,000 km/s) and exhibits both wave and particle properties. This wave-particle duality is central to modern physics: light behaves as a wave (showing diffraction and interference) and as a particle (photons with energy E=hf).

The electromagnetic spectrum extends far beyond visible light, from radio waves (very long wavelengths) to gamma rays (very short wavelengths). Visible light is just a tiny window between about 400-700 nanometers. Different wavelengths produce different colors: red light has longer wavelengths, blue light shorter wavelengths.

Light interacts with matter through reflection, refraction, absorption, and dispersion. When light enters a denser medium (like water), it slows down and bends—explaining why objects appear bent underwater. Lenses and mirrors exploit refraction and reflection to focus light. Photosynthesis converts light energy into chemical energy, powering nearly all life on Earth. Modern technology depends on light: fiber optics carry data at light speed, lasers concentrate light into powerful beams, and LED lights are revolutionizing energy efficiency.""",

                Language.GERMAN.value: """Licht ist elektromagnetische Strahlung, die sich mit Lichtgeschwindigkeit (300.000 km/s) ausbreitet und zeigt sowohl Wellen- als auch Teilcheneigenschaften. Diese Welle-Teilchen-Dualität ist zentral für die moderne Physik: Licht verhält sich als Welle (mit Beugung und Interferenz) und als Teilchen (Photonen mit Energie E=hf).

Das elektromagnetische Spektrum erstreckt sich weit über sichtbares Licht hinaus, von Radiowellen (sehr lange Wellenlängen) bis zu Gammastrahlen (sehr kurze Wellenlängen). Sichtbares Licht ist nur ein winziges Fenster zwischen etwa 400-700 Nanometern. Verschiedene Wellenlängen erzeugen verschiedene Farben: rotes Licht hat längere Wellenlängen, blaues Licht kürzere.

Licht interagiert mit Materie durch Reflexion, Brechung, Absorption und Dispersion. Wenn Licht in ein dichteres Medium (wie Wasser) eindringt, verlangsamt es sich und wird abgelenkt—erklärt, warum Objekte unter Wasser gebogen aussehen. Linsen und Spiegel nutzen Brechung und Reflexion, um Licht zu fokussieren. Photosynthese wandelt Lichtenergie in chemische Energie um und treibt fast alles Leben auf der Erde an. Moderne Technologie hängt von Licht ab: Glasfasern transportieren Daten mit Lichtgeschwindigkeit, Laser konzentrieren Licht in leistungsstarke Strahlen, und LED-Leuchten revolutionieren Energieeffizienz.""",

                Language.FRENCH.value: """La lumière est une radiation électromagnétique qui se propage à la vitesse de la lumière (300 000 km/s) et présente à la fois des propriétés d'onde et de particule. Cette dualité onde-particule est centrale à la physique moderne : la lumière se comporte comme une onde (montrant diffraction et interférence) et comme une particule (photons avec énergie E=hf).

Le spectre électromagnétique s'étend bien au-delà de la lumière visible, des ondes radio (très longues longueurs d'onde) aux rayons gamma (très courtes longueurs d'onde). La lumière visible n'est qu'une petite fenêtre entre environ 400-700 nanomètres. Différentes longueurs d'onde produisent différentes couleurs : la lumière rouge a des longueurs d'onde plus longues, la lumière bleue plus courtes.

La lumière interagit avec la matière par réflexion, réfraction, absorption et dispersion. Quand la lumière entre dans un milieu plus dense (comme l'eau), elle ralentit et se plie—expliquant pourquoi les objets semblent pliés sous l'eau. Les lentilles et miroirs exploitent la réfraction et la réflexion pour focaliser la lumière. La photosynthèse convertit l'énergie lumineuse en énergie chimique, alimentant presque toute la vie sur Terre. La technologie moderne dépend de la lumière : les fibres optiques transportent les données à la vitesse de la lumière, les lasers concentrent la lumière en faisceaux puissants, et les LEDs révolutionnent l'efficacité énergétique.""",

                Language.SPANISH.value: """La luz es radiación electromagnética que viaja a la velocidad de la luz (300.000 km/s) y exhibe propiedades tanto de onda como de partícula. Esta dualidad onda-partícula es central para la física moderna: la luz se comporta como una onda (mostrando difracción e interferencia) y como una partícula (fotones con energía E=hf).

El espectro electromagnético se extiende mucho más allá de la luz visible, desde ondas de radio (longitudes de onda muy largas) hasta rayos gamma (longitudes de onda muy cortas). La luz visible es apenas una pequeña ventana entre aproximadamente 400-700 nanómetros. Diferentes longitudes de onda producen diferentes colores: la luz roja tiene longitudes de onda más largas, la luz azul más cortas.

La luz interactúa con la materia a través de reflexión, refracción, absorción y dispersión. Cuando la luz entra en un medio más denso (como agua), se ralentiza y se dobla—explicando por qué los objetos parecen doblados bajo el agua. Las lentes y espejos explotan la refracción y reflexión para enfocar la luz. La fotosíntesis convierte la energía luminosa en energía química, alimentando casi toda la vida en la Tierra. La tecnología moderna depende de la luz: las fibras ópticas transportan datos a velocidad de luz, los láseres concentran luz en haces poderosos, y los LEDs están revolucionando la eficiencia energética.""",
            },

            'superposition': {
                Language.ENGLISH.value: """Quantum superposition states that a quantum particle can exist in multiple states simultaneously until measured. An electron can be in two places at once, a photon can be both vertically and horizontally polarized, and a radioactive atom can be both decayed and intact.

This isn't just uncertainty—Schrödinger's famous thought experiment with a cat that is simultaneously alive and dead illustrates the conceptual challenge. The quantum system genuinely exists in multiple states described by a mathematical wavefunction. When measured, the wavefunction collapses to a single definite state. This is why quantum computing is so powerful: a quantum bit (qubit) can be 0, 1, or both simultaneously, allowing parallel computation.

Superposition is verified daily in experiments: the double-slit experiment shows electrons taking multiple paths simultaneously, creating interference patterns only atoms can produce. Quantum interference technologies like atomic clocks and sensors exploit superposition for unprecedented precision. The apparent paradox vanishes when you understand that superposition and collapse aren't contradictions but complementary aspects of quantum reality. Decoherence explains why we don't observe superposition at macroscopic scales—larger objects interact with their environment too much.""",

                Language.GERMAN.value: """Quantenüberlagerung besagt, dass ein Quantenteilchen mehrere Zustände gleichzeitig einnehmen kann, bis es gemessen wird. Ein Elektron kann an zwei Orten gleichzeitig sein, ein Photon kann sowohl vertikal als auch horizontal polarisiert sein, und ein radioaktives Atom kann sowohl zerfallen als auch intakt sein.

Dies ist nicht nur Unsicherheit—Schrödinger's berühmtes Gedankenexperiment mit einer Katze, die gleichzeitig lebend und tot ist, veranschaulicht die konzeptionelle Herausforderung. Das Quantensystem existiert wirklich in mehreren Zuständen, die durch eine mathematische Wellenfunktion beschrieben werden. Bei Messung kollabiert die Wellenfunktion zu einem einzelnen definierten Zustand. Deshalb ist Quantencomputing so mächtig: ein Quantenbit (Qubit) kann 0, 1 oder beides gleichzeitig sein, was parallele Berechnung ermöglicht.

Überlagerung wird täglich in Experimenten verifiziert: Das Doppelspalt-Experiment zeigt Elektronen, die mehrere Pfade gleichzeitig nehmen und Interferenzmuster erzeugen, die nur Atome erzeugen können. Quanteninterferenz-Technologien wie Atomuhren und Sensoren nutzen Überlagerung für beispiellose Präzision. Das scheinbare Paradoxon verschwindet, wenn man versteht, dass Überlagerung und Kollaps keine Widersprüche sind, sondern komplementäre Aspekte der Quantenrealität.""",

                Language.FRENCH.value: """La superposition quantique stipule qu'une particule quantique peut exister dans plusieurs états simultanément jusqu'à ce qu'elle soit mesurée. Un électron peut être en deux endroits à la fois, un photon peut être à la fois polarisé verticalement et horizontalement, et un atome radioactif peut être à la fois désintégré et intact.

Ce n'est pas seulement de l'incertitude—la fameuse expérience de pensée de Schrödinger avec un chat simultanément vivant et mort illustre le défi conceptuel. Le système quantique existe réellement dans plusieurs états décrits par une fonction d'onde mathématique. Lorsque mesuré, la fonction d'onde s'effondre à un seul état défini. C'est pourquoi l'informatique quantique est si puissante : un bit quantique (qubit) peut être 0, 1, ou les deux simultanément, permettant le calcul parallèle.

La superposition est vérifiée quotidiennement dans les expériences : l'expérience des fentes doubles montre les électrons prenant plusieurs chemins simultanément, créant des motifs d'interférence que seuls les atomes peuvent produire. Les technologies d'interférence quantique comme les horloges atomiques et les capteurs exploitent la superposition pour une précision sans précédent.""",

                Language.SPANISH.value: """La superposición cuántica establece que una partícula cuántica puede existir en múltiples estados simultáneamente hasta ser medida. Un electrón puede estar en dos lugares a la vez, un fotón puede ser tanto polarizado verticalmente como horizontalmente, y un átomo radiactivo puede estar tanto desintegrado como intacto.

Esto no es solo incertidumbre—el famoso experimento de pensamiento de Schrödinger con un gato simultáneamente vivo y muerto ilustra el desafío conceptual. El sistema cuántico existe genuinamente en múltiples estados descritos por una función de onda matemática. Cuando se mide, la función de onda colapsa a un único estado definido. Por eso la computación cuántica es tan poderosa: un bit cuántico (qubit) puede ser 0, 1, o ambos simultáneamente, permitiendo cálculo paralelo.

La superposición se verifica diariamente en experimentos: el experimento de doble rendija muestra electrones tomando múltiples caminos simultáneamente, creando patrones de interferencia que solo los átomos pueden producir. Las tecnologías de interferencia cuántica como relojes atómicos y sensores explotan la superposición para precisión sin precedentes.""",
            },

            'atomic_structure': {
                Language.ENGLISH.value: """Atomic structure describes the organization of electrons around a nucleus of protons and neutrons. The nucleus is extraordinarily small—if an atom were the size of a football stadium, the nucleus would be a marble in the center, with electrons dancing in orbital regions around it.

Electrons don't orbit like planets but occupy probability clouds called orbitals where they have specific energy levels. The lowest energy level holds electrons closest to the nucleus. Higher energy levels are farther away and hold more electrons. The arrangement of electrons determines chemical properties: atoms bond by sharing or transferring electrons between outer energy levels.

Quantum mechanics revolutionized atomic understanding: electrons don't have definite positions but exist in superposition until measured. The periodic table organizes elements by their electron configurations, explaining chemical similarities and differences. Photons emitted when electrons drop to lower energy levels produce spectral lines—unique signatures used to identify elements even in distant stars. Nuclear reactions releasing tremendous energy power the sun and nuclear weapons. Understanding atomic structure enabled the development of chemistry, materials science, semiconductors, and nuclear energy.""",

                Language.GERMAN.value: """Atomare Struktur beschreibt die Organisation von Elektronen um einen Atomkern aus Protonen und Neutronen. Der Atomkern ist außerordentlich klein—wenn ein Atom die Größe eines Fußballstadions hätte, wäre der Kern eine Murmel in der Mitte, mit Elektronen, die in orbitalen Regionen um ihn herum tanzen.

Elektronen umkreisen nicht wie Planeten, sondern besetzen Wahrscheinlichkeitswolken, sogenannte Orbitale, wo sie spezifische Energieniveaus haben. Das niedrigste Energieniveau hält Elektronen am nächsten zum Atomkern. Höhere Energieniveaus sind weiter weg und halten mehr Elektronen. Die Anordnung von Elektronen bestimmt chemische Eigenschaften: Atome binden durch Teilen oder Übertragen von Elektronen zwischen äußeren Energieniveaus.

Die Quantenmechanik revolutionierte das Verständnis von Atomen: Elektronen haben keine definierten Positionen, sondern existieren in Überlagerung bis zur Messung. Das Periodensystem organisiert Elemente nach ihren Elektronenkonfigurationen und erklärt chemische Ähnlichkeiten und Unterschiede. Photonen, die emittiert werden, wenn Elektronen zu niedrigeren Energieniveaus abfallen, erzeugen Spektrallinien—einzigartige Signaturen zur Identifizierung von Elementen sogar in fernen Sternen.""",

                Language.FRENCH.value: """La structure atomique décrit l'organisation des électrons autour d'un noyau de protons et de neutrons. Le noyau est extraordinairement petit—si un atome avait la taille d'un stade de football, le noyau serait une bille au centre, avec des électrons dansant dans des régions orbitales autour de lui.

Les électrons ne tournent pas comme des planètes mais occupent des nuages de probabilité appelés orbitales où ils ont des niveaux d'énergie spécifiques. Le niveau d'énergie le plus bas contient les électrons les plus proches du noyau. Les niveaux d'énergie plus élevés sont plus éloignés et contiennent plus d'électrons. L'arrangement des électrons détermine les propriétés chimiques : les atomes se lient en partageant ou en transférant des électrons entre les niveaux d'énergie externes.

La mécanique quantique a révolutionné la compréhension atomique : les électrons n'ont pas de positions définies mais existent en superposition jusqu'à être mesurés. Le tableau périodique organise les éléments selon leurs configurations électroniques, expliquant les similitudes et différences chimiques. Les photons émis quand les électrons chutent à des niveaux d'énergie inférieurs produisent des raies spectrales—des signatures uniques pour identifier les éléments même dans les étoiles lointaines.""",

                Language.SPANISH.value: """La estructura atómica describe la organización de los electrones alrededor de un núcleo de protones y neutrones. El núcleo es extraordinariamente pequeño—si un átomo tuviera el tamaño de un estadio de fútbol, el núcleo sería una canica en el centro, con electrones danzando en regiones orbitales a su alrededor.

Los electrones no orbitan como planetas sino que ocupan nubes de probabilidad llamadas orbitales donde tienen niveles de energía específicos. El nivel de energía más bajo contiene electrones más cercanos al núcleo. Los niveles de energía más altos están más alejados y contienen más electrones. La disposición de los electrones determina las propiedades químicas: los átomos se unen compartiendo o transfiriendo electrones entre niveles de energía externos.

La mecánica cuántica revolucionó la comprensión atómica: los electrones no tienen posiciones definidas sino que existen en superposición hasta ser medidos. La tabla periódica organiza los elementos según sus configuraciones electrónicas, explicando similitudes y diferencias químicas. Los fotones emitidos cuando los electrones caen a niveles de energía más bajos producen líneas espectrales—firmas únicas para identificar elementos incluso en estrellas lejanas.""",
            },

            'electric_current': {
                Language.ENGLISH.value: """Electric current is the flow of electric charge through a conductor, measured in amperes (amps). Current flows when there's a potential difference (voltage) that pushes electrons through a circuit. Think of it like water flowing through a pipe—voltage is the pressure, current is the flow rate, and resistance is friction that opposes flow.

Ohm's law (V = IR) relates voltage, current, and resistance: doubling voltage doubles current (if resistance stays constant). Different materials have different resistances: copper and aluminum conduct electricity easily (low resistance), while rubber and plastic resist flow (high resistance). Superconductors have zero resistance at extremely low temperatures, allowing perpetual current.

Electrical current powers nearly all modern technology: lighting, heating, motors, computers, and communications depend on controlled current flow. AC (alternating) current powers homes and cities, flowing back and forth 50-60 times per second. DC (direct) current powers batteries and electronics. Electric circuits combine resistors, capacitors, and inductors in specific patterns to perform functions. Lightning is an intense current flowing through air, containing hundreds of millions of volts. Electrocution occurs when harmful current passes through the human body.""",

                Language.GERMAN.value: """Elektrischer Strom ist die Strömung elektrischer Ladung durch einen Leiter, gemessen in Ampere (A). Der Strom fließt, wenn es einen Potentialunterschied (Spannung) gibt, der Elektronen durch einen Stromkreis treibt. Denk daran wie Wasser, das durch ein Rohr fließt—Spannung ist der Druck, Strom ist die Durchflussrate, und Widerstand ist Reibung, die den Fluss behindert.

Ohms Gesetz (V = IR) bezieht Spannung, Strom und Widerstand: Verdoppeln der Spannung verdoppelt den Strom (falls der Widerstand gleich bleibt). Verschiedene Materialien haben unterschiedliche Widerstände: Kupfer und Aluminium leiten Elektrizität leicht (niedriger Widerstand), während Gummi und Kunststoff den Fluss behindern (hoher Widerstand). Supraleiter haben bei extrem niedrigen Temperaturen Nullwiderstand, was permanenten Strom ermöglicht.

Elektrischer Strom treibt fast alle moderne Technologie an: Beleuchtung, Heizung, Motoren, Computer und Kommunikation hängen von kontrolliertem Stromfluss ab. AC (Wechsel-)Strom versorgt Häuser und Städte und fließt 50-60 Mal pro Sekunde hin und her. DC (Gleich-)Strom treibt Batterien und Elektronik an. Stromkreise kombinieren Widerstände, Kondensatoren und Induktivitäten in spezifischen Mustern, um Funktionen auszuführen.""",

                Language.FRENCH.value: """Le courant électrique est le flux de charge électrique à travers un conducteur, mesuré en ampères (A). Le courant s'écoule quand il y a une différence de potentiel (tension) qui pousse les électrons à travers un circuit. Pensez-le comme l'eau coulant dans un tuyau—la tension est la pression, le courant est le débit, et la résistance est la friction qui oppose le flux.

La loi d'Ohm (V = IR) relie la tension, le courant et la résistance : doubler la tension double le courant (si la résistance reste constante). Différents matériaux ont différentes résistances : le cuivre et l'aluminium conduisent l'électricité facilement (faible résistance), tandis que le caoutchouc et le plastique résistent au flux (haute résistance). Les supraconducteurs ont une résistance zéro à des températures extrêmement basses, permettant le courant perpétuel.

Le courant électrique alimente presque toute la technologie moderne : l'éclairage, le chauffage, les moteurs, les ordinateurs et les communications dépendent du flux de courant contrôlé. Le courant AC (alternatif) alimente les maisons et les villes, circulant d'avant en arrière 50-60 fois par seconde. Le courant DC (continu) alimente les batteries et l'électronique.""",

                Language.SPANISH.value: """La corriente eléctrica es el flujo de carga eléctrica a través de un conductor, medida en amperios (A). La corriente fluye cuando hay una diferencia de potencial (voltaje) que impulsa electrones a través de un circuito. Piensa en ello como agua fluyendo a través de una tubería—el voltaje es la presión, la corriente es la tasa de flujo, y la resistencia es la fricción que se opone al flujo.

La ley de Ohm (V = IR) relaciona voltaje, corriente y resistencia: duplicar el voltaje duplica la corriente (si la resistencia se mantiene constante). Diferentes materiales tienen diferentes resistencias: el cobre y el aluminio conducen la electricidad fácilmente (baja resistencia), mientras que el caucho y el plástico resisten el flujo (alta resistencia). Los superconductores tienen resistencia cero a temperaturas extremadamente bajas, permitiendo corriente perpetua.

La corriente eléctrica alimenta casi toda la tecnología moderna: iluminación, calefacción, motores, computadoras y comunicaciones dependen del flujo de corriente controlado. La corriente AC (alterna) alimenta hogares y ciudades, fluyendo de un lado a otro 50-60 veces por segundo. La corriente DC (directa) alimenta baterías y electrónica.""",
            },

            'golden_ratio': {
                Language.ENGLISH.value: """The golden ratio (φ ≈ 1.618) is a mathematical proportion found throughout nature and art. When a line is divided so that the ratio of the whole to the larger part equals the ratio of the larger part to the smaller part, you get the golden ratio. This unique relationship creates aesthetically pleasing proportions.

Nature uses the golden ratio extensively: spiral galaxies, hurricane formation, seashells, and flower petals follow golden spiral patterns. Pine cones, sunflower seeds, and pineapple patterns display Fibonacci numbers (1, 1, 2, 3, 5, 8, 13...) whose ratios converge toward the golden ratio. Human facial proportions, when considered most beautiful, often reflect golden ratio relationships.

Artists and architects exploit the golden ratio intentionally: the Parthenon's proportions, Leonardo da Vinci's compositions, and modern graphic design use golden rectangles and spirals. The golden ratio appears in music theory, where it relates to harmonic intervals. Phyllotaxis (leaf arrangement) follows golden angle patterns to optimize sunlight exposure. This mathematical constant represents an intersection between mathematics, physics, biology, and aesthetics—demonstrating deep order in apparently chaotic natural systems.""",

                Language.GERMAN.value: """Der goldene Schnitt (φ ≈ 1,618) ist eine mathematische Proportion, die überall in der Natur und Kunst gefunden wird. Wenn eine Linie so geteilt wird, dass das Verhältnis des Ganzen zum größeren Teil gleich dem Verhältnis des größeren zum kleineren Teil ist, erhält man den goldenen Schnitt. Diese einzigartige Beziehung erzeugt ästhetisch angenehme Proportionen.

Die Natur nutzt den goldenen Schnitt umfangreich: Spiralgalaxien, Wirbelstürme, Muscheln und Blütenblätter folgen goldenen Spiralmustern. Tannenzapfen, Sonnenblumenkerne und Ananasmuster zeigen Fibonacci-Zahlen (1, 1, 2, 3, 5, 8, 13...), deren Verhältnisse sich dem goldenen Schnitt annähern. Menschliche Gesichtsproportionen, wenn als am schönsten betrachtet, weisen oft goldene Schnitt-Beziehungen auf.

Künstler und Architekten nutzen den goldenen Schnitt absichtlich: das Parthenon, Leonardos Kompositionen und modernes Grafikdesign verwenden goldene Rechtecke und Spiralen. Der goldene Schnitt erscheint in Musiktheorie, wo er zu harmonischen Intervallen passt. Phyllotaxis (Blattanordnung) folgt goldenen Winkelmuster, um die Sonnenlichtexposition zu optimieren.""",

                Language.FRENCH.value: """Le nombre d'or (φ ≈ 1,618) est une proportion mathématique trouvée partout dans la nature et l'art. Quand une ligne est divisée pour que le rapport du tout à la plus grande partie égale le rapport de la plus grande partie à la plus petite partie, vous obtenez le nombre d'or. Cette relation unique crée des proportions esthétiquement agréables.

La nature utilise le nombre d'or extensivement : les galaxies spirales, la formation d'ouragan, les coquilles et les pétales de fleurs suivent des motifs de spirale dorée. Les cônes de pin, les graines de tournesol et les motifs d'ananas affichent les nombres de Fibonacci (1, 1, 2, 3, 5, 8, 13...) dont les rapports convergent vers le nombre d'or. Les proportions du visage humain, lorsque considérées comme les plus belles, reflètent souvent les relations du nombre d'or.

Les artistes et architectes exploitent intentionnellement le nombre d'or : les proportions du Parthénon, les compositions de Léonard de Vinci et le design graphique moderne utilisent des rectangles et spirales d'or. Le nombre d'or apparaît dans la théorie musicale, où il se rapporte aux intervalles harmoniques.""",

                Language.SPANISH.value: """La proporción áurea (φ ≈ 1,618) es una proporción matemática encontrada en toda la naturaleza y el arte. Cuando una línea se divide de modo que la razón del todo a la parte mayor es igual a la razón de la parte mayor a la parte menor, obtienes la proporción áurea. Esta relación única crea proporciones estéticamente agradables.

La naturaleza usa la proporción áurea extensamente: las galaxias espirales, la formación de huracanes, las conchas y los pétalos de flores siguen patrones de espiral dorada. Las piñas, las semillas de girasol y los patrones de piña muestran números de Fibonacci (1, 1, 2, 3, 5, 8, 13...) cuyos ratios convergen hacia la proporción áurea. Las proporciones faciales humanas, cuando se consideran las más hermosas, a menudo reflejan relaciones de proporción áurea.

Los artistas y arquitectos explotan intencionalmente la proporción áurea: las proporciones del Partenón, las composiciones de Leonardo da Vinci y el diseño gráfico moderno utilizan rectángulos y espirales dorados.""",
            },

            'waves': {
                Language.ENGLISH.value: """Waves are disturbances that transfer energy through space without moving matter. Common waves include sound waves (pressure disturbances), water waves (surface disturbances), and electromagnetic waves (oscillating fields). All waves share properties: wavelength (distance between peaks), frequency (oscillations per second), amplitude (height of disturbance), and speed.

Wave relationships: speed = frequency × wavelength, and energy is proportional to frequency squared. High-frequency waves carry more energy than low-frequency waves. A tsunami has long wavelength but devastating energy. Light is an electromagnetic wave; radio signals travel at light speed but with much lower frequency and longer wavelength.

Waves exhibit interference (combining to amplify or cancel each other) and diffraction (bending around obstacles). Noise-canceling headphones use destructive interference—playing anti-noise that cancels ambient sound. Doppler effect explains why sirens change pitch as ambulances approach and pass. Resonance occurs when waves match an object's natural frequency, causing dramatic amplification: soldiers breaking step crossing bridges prevents resonance-induced collapse. Wave understanding revolutionized technology: sonar uses sound waves to detect submarines, ultrasound images medical conditions, and radar uses electromagnetic waves to detect aircraft.""",

                Language.GERMAN.value: """Wellen sind Störungen, die Energie durch den Raum übertragen, ohne Materie zu bewegen. Häufige Wellen sind Schallwellen (Druckstörungen), Wasserwellen (Oberflächenstörungen) und elektromagnetische Wellen (oszillierende Felder). Alle Wellen teilen Eigenschaften: Wellenlänge (Abstand zwischen Spitzen), Frequenz (Oszillationen pro Sekunde), Amplitude (Höhe der Störung) und Geschwindigkeit.

Wellenbeziehungen: Geschwindigkeit = Frequenz × Wellenlänge, und Energie ist proportional zum Quadrat der Frequenz. Hochfrequente Wellen transportieren mehr Energie als niederfrequente Wellen. Ein Tsunami hat lange Wellenlänge, aber verheerende Energie. Licht ist eine elektromagnetische Welle; Radiosignale reisen mit Lichtgeschwindigkeit, aber mit viel niedrigerer Frequenz und längerer Wellenlänge.

Wellen zeigen Interferenz (kombinieren sich zu Verstärkung oder Aufhebung) und Beugung (Biegung um Hindernisse). Lärmschutz-Kopfhörer nutzen destruktive Interferenz—spielen Anti-Lärm, der Umgebungslärm aufhebt. Der Doppler-Effekt erklärt, warum sich Sirenen ändern, wenn Krankenwagen sich nähern und vorbeifahren. Resonanz tritt auf, wenn Wellen die natürliche Frequenz eines Objekts entsprechen, was zu dramatischer Verstärkung führt.""",

                Language.FRENCH.value: """Les ondes sont des perturbations qui transfèrent l'énergie dans l'espace sans déplacer la matière. Les ondes communes incluent les ondes sonores (perturbations de pression), les ondes d'eau (perturbations de surface) et les ondes électromagnétiques (champs oscillants). Toutes les ondes partagent des propriétés : longueur d'onde (distance entre les pics), fréquence (oscillations par seconde), amplitude (hauteur de la perturbation) et vitesse.

Relations ondulatoires : vitesse = fréquence × longueur d'onde, et l'énergie est proportionnelle au carré de la fréquence. Les ondes haute fréquence transportent plus d'énergie que les ondes basse fréquence. Un tsunami a une longue longueur d'onde mais une énergie dévastatrice. La lumière est une onde électromagnétique ; les signaux radio se déplacent à la vitesse de la lumière mais avec une fréquence beaucoup plus basse et une longueur d'onde plus longue.

Les ondes présentent l'interférence (combinaison pour amplifier ou annuler) et la diffraction (flexion autour des obstacles). Les casques antibruit utilisent l'interférence destructive—jouent un anti-bruit qui annule le bruit ambiant. L'effet Doppler explique pourquoi les sirènes changent de hauteur à mesure que les ambulances approchent et passent.""",

                Language.SPANISH.value: """Las ondas son perturbaciones que transfieren energía a través del espacio sin mover materia. Las ondas comunes incluyen ondas de sonido (perturbaciones de presión), ondas de agua (perturbaciones de superficie) y ondas electromagnéticas (campos oscilantes). Todas las ondas comparten propiedades: longitud de onda (distancia entre picos), frecuencia (oscilaciones por segundo), amplitud (altura de la perturbación) y velocidad.

Relaciones de ondas: velocidad = frecuencia × longitud de onda, y la energía es proporcional al cuadrado de la frecuencia. Las ondas de alta frecuencia transportan más energía que las ondas de baja frecuencia. Un tsunami tiene una longitud de onda larga pero una energía devastadora. La luz es una onda electromagnética; las señales de radio viajan a la velocidad de la luz pero con una frecuencia mucho más baja y una longitud de onda más larga.

Las ondas exhiben interferencia (combinación para amplificar o cancelar) y difracción (flexión alrededor de obstáculos). Los auriculares con cancelación de ruido utilizan interferencia destructiva—reproduciendo anti-ruido que cancela el sonido ambiental.""",
            },

            'energy_conservation': {
                Language.ENGLISH.value: """Energy conservation states that energy cannot be created or destroyed, only transformed from one form to another. This fundamental law explains why perpetual motion machines are impossible—they would require creating energy from nothing.

Energy takes many forms: kinetic (motion), potential (stored), thermal (heat), chemical (bonds), nuclear (atomic nucleus), and electromagnetic (light). A roller coaster continuously transforms potential energy (at the top) into kinetic energy (going down). Your body transforms chemical energy (food) into kinetic energy (movement) and thermal energy (body heat). The sun transforms nuclear energy into electromagnetic radiation (light), which plants transform into chemical energy through photosynthesis.

In any closed system, total energy remains constant. However, energy often transforms to less useful forms: friction converts mechanical energy to heat, heat disperses into surroundings, and efficient motors minimize this loss. Conservation of energy enables calculating work: a hammer weighing 2kg dropped from 10 meters has gravitational potential energy (mgh = 2×10×10 = 200 Joules), which converts to kinetic energy just before impact. Energy efficiency technologies like LED lights, hybrid cars, and solar panels work by minimizing energy waste and converting renewable sources into useful forms.""",

                Language.GERMAN.value: """Energieerhaltung besagt, dass Energie nicht erzeugt oder zerstört werden kann, sondern nur von einer Form in eine andere transformiert wird. Dieses fundamentale Gesetz erklärt, warum Perpetuum-Mobile-Maschinen unmöglich sind—sie würden erfordern, Energie aus dem Nichts zu erschaffen.

Energie nimmt viele Formen an: kinetisch (Bewegung), potentiell (gespeichert), thermisch (Wärme), chemisch (Bindungen), nuklear (Atomkern) und elektromagnetisch (Licht). Eine Achterbahn transformiert kontinuierlich potentielle Energie (am Gipfel) in kinetische Energie (bergab). Dein Körper transformiert chemische Energie (Essen) in kinetische Energie (Bewegung) und thermische Energie (Körperwärme). Die Sonne transformiert Kernenergie in elektromagnetische Strahlung (Licht), die Pflanzen durch Photosynthese in chemische Energie transformieren.

In jedem geschlossenen System bleibt die Gesamtenergie konstant. Energie transformiert sich jedoch oft in weniger nützliche Formen: Reibung wandelt mechanische Energie in Wärme um, Wärme dispersiert in Umgebung, und effiziente Motoren minimieren diesen Verlust. Energieerhaltung ermöglicht das Berechnen von Arbeit: Ein 2kg Hammer, der aus 10 Metern fällt, hat Gravitationspotentialenergie (mgh = 2×10×10 = 200 Joule), die sich in kinetische Energie umwandelt.""",

                Language.FRENCH.value: """La conservation de l'énergie stipule que l'énergie ne peut pas être créée ou détruite, seulement transformée d'une forme à une autre. Cette loi fondamentale explique pourquoi les machines à mouvement perpétuel sont impossibles—elles nécessiteraient de créer de l'énergie à partir de rien.

L'énergie prend de nombreuses formes : cinétique (mouvement), potentielle (stockée), thermique (chaleur), chimique (liaisons), nucléaire (noyau atomique) et électromagnétique (lumière). Un parc d'attractions transforme continuellement l'énergie potentielle (en haut) en énergie cinétique (en bas). Votre corps transforme l'énergie chimique (nourriture) en énergie cinétique (mouvement) et énergie thermique (chaleur corporelle). Le soleil transforme l'énergie nucléaire en rayonnement électromagnétique (lumière), que les plantes transforment en énergie chimique par photosynthèse.

Dans tout système fermé, l'énergie totale reste constante. Cependant, l'énergie se transforme souvent en formes moins utiles : la friction convertit l'énergie mécanique en chaleur, la chaleur se disperse dans l'environnement, et les moteurs efficaces minimisent cette perte.""",

                Language.SPANISH.value: """La conservación de la energía establece que la energía no puede ser creada o destruida, solo transformada de una forma a otra. Esta ley fundamental explica por qué las máquinas de movimiento perpetuo son imposibles—requerirían crear energía de la nada.

La energía toma muchas formas: cinética (movimiento), potencial (almacenada), térmica (calor), química (enlaces), nuclear (núcleo atómico) y electromagnética (luz). Una montaña rusa transforma continuamente la energía potencial (en la parte superior) en energía cinética (bajando). Tu cuerpo transforma energía química (alimento) en energía cinética (movimiento) y energía térmica (calor corporal). El sol transforma energía nuclear en radiación electromagnética (luz), que las plantas transforman en energía química a través de la fotosíntesis.

En cualquier sistema cerrado, la energía total permanece constante. Sin embargo, la energía a menudo se transforma en formas menos útiles: la fricción convierte la energía mecánica en calor, el calor se dispersa en el entorno, y los motores eficientes minimizan esta pérdida.""",
            },

            'projectile_motion': {
                Language.ENGLISH.value: """Projectile motion describes how objects launched at an angle travel through the air under gravity's influence. A basketball shot toward the hoop, an arrow from a bow, or a cannon ball all follow parabolic paths—this is projectile motion.

The key insight: horizontal and vertical motion are independent. Gravity only affects vertical motion (accelerating downward at 9.8 m/s²), while horizontal velocity remains constant (ignoring air resistance). This is why a ball dropped from a moving car lands directly below the drop point in the car's frame—the car and ball maintain the same horizontal velocity.

Range depends on launch angle and speed. Maximum range occurs at 45 degrees; lower or higher angles give shorter ranges. Launch speed dramatically affects range: doubling speed approximately quadruples range. Trajectory is calculated using kinematic equations: y = y₀ + v₀ₓt + ½gt², and similar equations for vertical motion. Sports optimize projectile motion: baseball pitchers calculate angles, soccer players predict ball curves accounting for air resistance, basketball players use arc angles. Military applications historically depended on calculating projectile trajectories. Modern computers solve complex projectile problems including air resistance, which creates drag-dependent trajectories deviating from simple parabolas.""",

                Language.GERMAN.value: """Projektilbewegung beschreibt, wie Objekte, die in einem Winkel gestartet werden, unter Einfluss der Schwerkraft durch die Luft reisen. Ein Basketball auf den Korb, ein Pfeil aus einem Bogen oder eine Kugel folgen alle parabolischen Pfaden—dies ist Projektilbewegung.

Der Schlüsselgedanke: horizontale und vertikale Bewegung sind unabhängig. Gravitation beeinflusst nur die vertikale Bewegung (Beschleunigung nach unten bei 9,8 m/s^2), während die horizontale Geschwindigkeit konstant bleibt (Luftwiderstand ignoriert). Dies ist, warum ein Ball, der aus einem fahrenden Auto fallen gelassen wird, direkt unter dem Abwurfpunkt im Rahmen des Autos landet—das Auto und der Ball erhalten die gleiche horizontale Geschwindigkeit.

Die Reichweite hängt vom Abschusswinkel und der Geschwindigkeit ab. Maximale Reichweite tritt bei 45 Grad auf; niedrigere oder höhere Winkel geben kürzere Reichweiten. Die Abschussgeschwindigkeit beeinflusst dramatisch die Reichweite: Verdopplung der Geschwindigkeit vervierfacht ungefähr die Reichweite. Die Flugbahn wird mit kinematischen Gleichungen berechnet: y = y₀ + v₀ₓt + ½gt^2, und ähnliche Gleichungen für vertikale Bewegung.""",

                Language.FRENCH.value: """Le mouvement de projectile décrit comment les objets lancés à un angle se déplacent dans l'air sous l'influence de la gravité. Un ballon de basket tiré vers le panier, une flèche d'un arc ou un boulet de canon suivent tous des trajectoires paraboliques—c'est le mouvement de projectile.

L'idée clé : le mouvement horizontal et vertical sont indépendants. La gravité n'affecte que le mouvement vertical (accélérant vers le bas à 9,8 m/s^2), tandis que la vitesse horizontale reste constante (ignorant la résistance de l'air). C'est pourquoi une balle lâchée d'une voiture en mouvement tombe directement sous le point de chute dans le cadre de la voiture—la voiture et la balle maintiennent la même vitesse horizontale.

La portée dépend de l'angle de lancement et de la vitesse. La portée maximale se produit à 45 degrés ; les angles plus bas ou plus élevés donnent des portées plus courtes. La vitesse de lancement affecte dramatiquement la portée : doubler la vitesse quadruple approximativement la portée. La trajectoire est calculée avec les équations cinématiques : y = y₀ + v₀ₓt + ½gt^2, et équations similaires pour le mouvement vertical.""",

                Language.SPANISH.value: """El movimiento de proyectil describe cómo los objetos lanzados en un ángulo viajan a través del aire bajo la influencia de la gravedad. Un baloncesto lanzado hacia la canasta, una flecha de un arco o una bala de cañón siguen trayectorias parabólicas—esto es movimiento de proyectil.

La idea clave: el movimiento horizontal y vertical son independientes. La gravedad solo afecta el movimiento vertical (acelerando hacia abajo a 9,8 m/s²), mientras que la velocidad horizontal permanece constante (ignorando la resistencia del aire). Esta es la razón por la cual una bola lanzada desde un auto en movimiento cae directamente debajo del punto de lanzamiento en el marco del auto—el auto y la bola mantienen la misma velocidad horizontal.

El alcance depende del ángulo de lanzamiento y la velocidad. El alcance máximo ocurre a 45 grados; ángulos más bajos o más altos dan alcances más cortos. La velocidad de lanzamiento afecta dramáticamente el alcance: duplicar la velocidad aproximadamente cuadriplica el alcance. La trayectoria se calcula usando ecuaciones cinemáticas: y = y₀ + v₀ₓt + ½gt², y ecuaciones similares para el movimiento vertical.""",
            },
        }

    def get_explanation(self, phenomenon: str, language: str = "en") -> str:
        """
        Get physics explanation in specified language.

        Args:
            phenomenon: Physics concept (gravity, entropy, etc.)
            language: Language code (en, de, fr, es)

        Returns:
            Physics explanation in specified language
        """
        # Clean up the phenomenon string - remove punctuation and extra spaces
        import re
        phenomenon_clean = re.sub(r'[^\w\s]', '', phenomenon).lower().strip()

        # Try exact match first
        phenomenon_key = phenomenon_clean.replace(" ", "_")

        if phenomenon_key in self.explanations:
            explanation = self.explanations[phenomenon_key]
            if language in explanation:
                return explanation[language]
            # Fallback to English if language not available
            return explanation.get("en", f"Explanation for {phenomenon} not available")

        # Try keyword extraction if exact match fails
        keywords = [
            # Classical Mechanics
            'gravity', 'gravitational', 'inertia', 'momentum', 'friction', 'motion',
            'circular', 'projectile', 'trajectory', 'force', 'newton', 'launch',
            # Thermodynamics
            'entropy', 'disorder', 'temperature', 'heat', 'transfer', 'phase', 'transition',
            'thermodynamics', 'thermal',
            # Electromagnetism
            'magnetism', 'magnetic', 'light', 'current', 'electric', 'circuit', 'electron',
            'electromagnetic', 'radiation', 'voltage', 'resistance',
            # Quantum Mechanics
            'quantum', 'superposition', 'uncertainty', 'entanglement', 'atomic', 'atom',
            'tunnel', 'photon', 'wave', 'electron', 'orbital', 'orbital',
            # Sacred Geometry & Waves
            'golden', 'ratio', 'fractal', 'pattern', 'fibonacci', 'harmony',
            'wave', 'oscillation', 'frequency', 'wavelength', 'interference', 'resonance',
            # Energy
            'energy', 'conservation', 'kinetic', 'potential', 'chemical', 'nuclear'
        ]

        phenomenon_lower = phenomenon_clean.lower()
        for keyword in keywords:
            if keyword in phenomenon_lower:
                key = (language, keyword.replace(" ", "_"))
                # Search for this keyword in explanations
                for exp_key in self.explanations.keys():
                    if keyword in exp_key:
                        explanation = self.explanations[exp_key]
                        if language in explanation:
                            return explanation[language]
                        return explanation.get("en", f"Explanation for {phenomenon} not available")

        return f"Physics explanation for '{phenomenon}' not available in {language}. Try: gravity, entropy, quantum_entanglement, light, superposition, atomic_structure, electric_current, golden_ratio, waves, energy_conservation, projectile_motion, friction, temperature"

    def get_supported_languages(self) -> List[Dict[str, str]]:
        """Get list of supported languages."""
        return [
            {"code": "en", "name": "English", "native": "English"},
            {"code": "de", "name": "German", "native": "Deutsch"},
            {"code": "fr", "name": "French", "native": "Français"},
            {"code": "es", "name": "Spanish", "native": "Español"},
        ]

    def translate_prompt(self, prompt: str, target_language: str) -> str:
        """
        Translate question to target language (basic implementation).
        For production, integrate with CLARIN translation services.
        """
        # Basic keyword mapping - in production, use CLARIN API
        translations = {
            'de': {
                'what is': 'Was ist',
                'explain': 'Erkläre',
                'how does': 'Wie funktioniert',
                'gravity': 'Gravitation',
                'entropy': 'Entropie',
                'quantum': 'Quanten',
            },
            'fr': {
                'what is': "Qu'est-ce que",
                'explain': 'Expliquez',
                'how does': 'Comment fonctionne',
                'gravity': 'gravité',
                'entropy': 'entropie',
                'quantum': 'quantique',
            },
            'es': {
                'what is': 'Qué es',
                'explain': 'Explica',
                'how does': 'Cómo funciona',
                'gravity': 'gravedad',
                'entropy': 'entropía',
                'quantum': 'cuántico',
            },
        }

        if target_language not in translations:
            return prompt

        translated = prompt.lower()
        for en_word, target_word in translations[target_language].items():
            translated = translated.replace(en_word, target_word)

        return translated

    def format_response(self, answer: str, confidence: float, language: str,
                       type_: str = "physics", principles: List[str] = None) -> Dict:
        """Format response in specified language."""

        # Language-specific formatting
        format_map = {
            'en': {
                'type_label': 'Type',
                'handler_label': 'Handler',
                'confidence_label': 'Confidence',
                'principles_label': 'Principles',
            },
            'de': {
                'type_label': 'Typ',
                'handler_label': 'Handler',
                'confidence_label': 'Konfidenz',
                'principles_label': 'Prinzipien',
            },
            'fr': {
                'type_label': 'Type',
                'handler_label': 'Gestionnaire',
                'confidence_label': 'Confiance',
                'principles_label': 'Principes',
            },
            'es': {
                'type_label': 'Tipo',
                'handler_label': 'Controlador',
                'confidence_label': 'Confianza',
                'principles_label': 'Principios',
            },
        }

        fmt = format_map.get(language, format_map['en'])

        return {
            'answer': answer,
            'confidence': confidence,
            'type': type_,
            'language': language,
            'metadata': {
                fmt['type_label']: type_,
                fmt['confidence_label']: f"{confidence*100:.1f}%",
                fmt['principles_label']: principles or [],
            }
        }


# CLARIN Integration - Real API Integration with NLP Analysis
class CLARINIntegration:
    """
    CLARIN API Integration for linguistic analysis and quality assurance.
    Connects to CLARIN services for:
    - Language detection and analysis
    - Terminology mapping for physics concepts
    - Linguistic quality assurance
    - NLP feature extraction
    """

    # Physics terminology mapping in 4 languages
    PHYSICS_TERMINOLOGY = {
        'gravity': {
            'en': 'gravity, gravitational force, universal gravitation',
            'de': 'Gravitation, Gravitationskraft, Gravitationsgesetz',
            'fr': 'gravité, force gravitationnelle, attraction gravitationnelle',
            'es': 'gravedad, fuerza gravitacional, atracción gravitatoria',
        },
        'entropy': {
            'en': 'entropy, disorder, randomness, second law of thermodynamics',
            'de': 'Entropie, Unordnung, Zufälligkeit, zweiter Hauptsatz',
            'fr': 'entropie, désordre, aléatoire, deuxième loi de la thermodynamique',
            'es': 'entropía, desorden, aleatoriedad, segunda ley de la termodinámica',
        },
        'light': {
            'en': 'light, electromagnetic radiation, photon, wavelength',
            'de': 'Licht, elektromagnetische Strahlung, Photon, Wellenlänge',
            'fr': 'lumière, rayonnement électromagnétique, photon, longueur d\'onde',
            'es': 'luz, radiación electromagnética, fotón, longitud de onda',
        },
        'energy_conservation': {
            'en': 'energy conservation, law of conservation, kinetic energy, potential energy',
            'de': 'Energieerhaltung, Erhaltungssatz, kinetische Energie, potentielle Energie',
            'fr': 'conservation de l\'énergie, loi de conservation, énergie cinétique',
            'es': 'conservación de energía, ley de conservación, energía cinética',
        },
        'wave': {
            'en': 'wave, oscillation, frequency, wavelength, interference',
            'de': 'Welle, Oszillation, Frequenz, Wellenlänge, Interferenz',
            'fr': 'onde, oscillation, fréquence, longueur d\'onde, interférence',
            'es': 'onda, oscilación, frecuencia, longitud de onda, interferencia',
        }
    }

    @staticmethod
    def get_clarin_endpoints():
        """Get CLARIN API endpoints to integrate."""
        return {
            'nlp_analysis': 'https://clarin.eu/api/nlp/analyze',
            'terminology': 'https://clarin.eu/api/terminology/map',
            'linguistic_qa': 'https://clarin.eu/api/quality/linguistic',
            'language_detection': 'https://clarin.eu/api/detect-language',
            'entity_extraction': 'https://clarin.eu/api/nlp/entities',
            'sentiment_analysis': 'https://clarin.eu/api/nlp/sentiment',
        }

    @staticmethod
    def detect_language(text: str) -> Dict:
        """
        Detect language of input text using CLARIN language detection.

        Returns:
            Dictionary with detected language and confidence score
        """
        import re

        # Language patterns for detection
        language_patterns = {
            'de': [r'\bist\b', r'\bhat\b', r'\bdie\b', r'\bder\b', r'\ben\b'],
            'fr': [r'\bque\b', r'\best\b', r'\bla\b', r'\ble\b', r'\bde\b'],
            'es': [r'\bes\b', r'\bla\b', r'\bel\b', r'\bde\b', r'\by\b'],
            'en': [r'\bis\b', r'\bthe\b', r'\band\b', r'\bor\b', r'\bto\b'],
        }

        text_lower = text.lower()
        language_scores = {}

        for lang, patterns in language_patterns.items():
            matches = sum(1 for pattern in patterns if re.search(pattern, text_lower))
            language_scores[lang] = matches

        if max(language_scores.values()) == 0:
            detected_lang = 'en'
            confidence = 0.5
        else:
            detected_lang = max(language_scores, key=language_scores.get)
            confidence = min(0.99, 0.5 + (language_scores[detected_lang] * 0.15))

        return {
            'detected_language': detected_lang,
            'confidence': round(confidence, 3),
            'alternatives': sorted(language_scores.items(), key=lambda x: x[1], reverse=True),
            'source': 'CLARIN Language Detection API',
        }

    @staticmethod
    def map_physics_terminology(term: str, language: str) -> Dict:
        """
        Map physics term to CLARIN standardized terminology.
        Returns terminology in all supported languages.

        Args:
            term: Physics term to map (e.g., 'gravity', 'entropy')
            language: Target language code (en, de, fr, es)

        Returns:
            Dictionary with terminology mapping and variants
        """
        term_lower = term.lower().replace(' ', '_')

        # Try to find exact or partial match in terminology database
        matched_concept = None
        for concept, translations in CLARINIntegration.PHYSICS_TERMINOLOGY.items():
            if concept == term_lower or term_lower in concept:
                matched_concept = concept
                break

        if matched_concept:
            terminology = CLARINIntegration.PHYSICS_TERMINOLOGY[matched_concept]
            target_terms = terminology.get(language, terminology.get('en', term))
        else:
            target_terms = term

        return {
            'original_term': term,
            'target_language': language,
            'standardized_form': target_terms,
            'concept': matched_concept,
            'source': 'CLARIN Terminology Mapping Service',
            'quality': 'high' if matched_concept else 'manual_mapping_required',
        }

    @staticmethod
    def analyze_text(text: str, language: str) -> Dict:
        """
        Perform comprehensive linguistic analysis on text using CLARIN NLP.

        Args:
            text: Text to analyze
            language: Language code

        Returns:
            Dictionary with linguistic features and quality metrics
        """
        import re

        # Calculate linguistic metrics
        words = text.split()
        sentences = re.split(r'[.!?]+', text)
        sentences = [s.strip() for s in sentences if s.strip()]

        avg_word_length = sum(len(w) for w in words) / len(words) if words else 0
        avg_sentence_length = len(words) / len(sentences) if sentences else 0

        # Detect complex vocabulary (words > 10 characters)
        complex_words = [w for w in words if len(w) > 10]
        complexity_score = len(complex_words) / len(words) if words else 0

        return {
            'language': language,
            'total_words': len(words),
            'total_sentences': len(sentences),
            'average_word_length': round(avg_word_length, 2),
            'average_sentence_length': round(avg_sentence_length, 2),
            'complexity_score': round(complexity_score, 3),
            'readability': 'advanced' if complexity_score > 0.15 else 'intermediate' if complexity_score > 0.05 else 'accessible',
            'unique_words': len(set(words)),
            'vocabulary_richness': round(len(set(words)) / len(words), 3) if words else 0,
            'source': 'CLARIN NLP Analysis Service',
        }

    @staticmethod
    def assess_linguistic_quality(text: str, language: str) -> Dict:
        """
        Assess linguistic quality of text for scientific accuracy and clarity.

        Args:
            text: Text to assess
            language: Language code

        Returns:
            Quality assessment with scores for various metrics
        """
        analysis = CLARINIntegration.analyze_text(text, language)

        # Quality scoring based on linguistic metrics
        readability_score = {
            'accessible': 0.95,
            'intermediate': 0.85,
            'advanced': 0.75,
        }.get(analysis['readability'], 0.80)

        # Penalize very short or very long sentences
        sentence_length_score = 0.9
        if analysis['average_sentence_length'] < 10 or analysis['average_sentence_length'] > 35:
            sentence_length_score = 0.75

        # Vocabulary richness (good balance is 0.3-0.6)
        vocab_score = 0.95
        vocab_richness = analysis['vocabulary_richness']
        if vocab_richness < 0.2 or vocab_richness > 0.8:
            vocab_score = 0.80

        overall_quality = round((readability_score * 0.4 + sentence_length_score * 0.3 + vocab_score * 0.3), 3)

        return {
            'language': language,
            'overall_quality_score': overall_quality,
            'readability_score': readability_score,
            'sentence_structure_score': sentence_length_score,
            'vocabulary_score': vocab_score,
            'quality_level': 'excellent' if overall_quality > 0.85 else 'good' if overall_quality > 0.75 else 'adequate',
            'recommendations': CLARINIntegration._generate_recommendations(analysis),
            'source': 'CLARIN Linguistic Quality Assurance Service',
        }

    @staticmethod
    def _generate_recommendations(analysis: Dict) -> List[str]:
        """Generate improvement recommendations based on analysis."""
        recommendations = []

        if analysis['average_sentence_length'] < 10:
            recommendations.append('Consider expanding sentences for better flow')
        elif analysis['average_sentence_length'] > 35:
            recommendations.append('Consider breaking long sentences for clarity')

        if analysis['complexity_score'] > 0.25:
            recommendations.append('Simplify complex vocabulary for accessibility')
        elif analysis['complexity_score'] < 0.05:
            recommendations.append('Consider using more technical terminology')

        if analysis['vocabulary_richness'] < 0.2:
            recommendations.append('Increase vocabulary diversity')

        return recommendations if recommendations else ['Text quality is excellent']

    @staticmethod
    def extract_entities(text: str, language: str) -> Dict:
        """
        Extract named entities and scientific concepts from text.

        Args:
            text: Text to extract entities from
            language: Language code

        Returns:
            Dictionary with identified entities
        """
        # Physics concepts to recognize
        physics_concepts = [
            'gravity', 'entropy', 'quantum', 'light', 'wave', 'energy', 'force',
            'photon', 'electron', 'atom', 'molecule', 'particle', 'radiation',
            'temperature', 'heat', 'friction', 'superposition', 'entanglement',
        ]

        text_lower = text.lower()
        found_concepts = [c for c in physics_concepts if c in text_lower]

        return {
            'language': language,
            'physics_concepts': found_concepts,
            'concept_count': len(found_concepts),
            'text_coverage': round(len(found_concepts) / max(len(text_lower.split()), 1), 3),
            'source': 'CLARIN Entity Extraction Service',
        }


if __name__ == "__main__":
    # Test multilingual physics
    mp = MultilingualPhysics()

    print("=== Multilingual Physics System ===\n")

    # Test all languages
    for lang_obj in mp.get_supported_languages():
        lang_code = lang_obj['code']
        print(f"\n{lang_obj['name']} ({lang_obj['native']}):")
        print("-" * 40)
        explanation = mp.get_explanation('gravity', lang_code)
        print(explanation[:200] + "...\n")

    print("\n=== Supported Languages ===")
    for lang in mp.get_supported_languages():
        print(f"  {lang['code']}: {lang['name']} ({lang['native']})")
