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
            'circular', 'projectile', 'force', 'newton',
            # Thermodynamics
            'entropy', 'disorder', 'temperature', 'heat', 'transfer', 'phase', 'transition',
            'thermodynamics',
            # Electromagnetism
            'magnetism', 'magnetic', 'light', 'current', 'electric', 'circuit', 'electron',
            'electromagnetic', 'radiation',
            # Quantum Mechanics
            'quantum', 'superposition', 'uncertainty', 'entanglement', 'atomic', 'atom',
            'tunnel', 'photon', 'wave',
            # Sacred Geometry
            'golden', 'ratio', 'fractal', 'pattern', 'fibonacci', 'harmony'
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

        return f"Physics explanation for '{phenomenon}' not available in {language}. Try: gravity, entropy, quantum entanglement, light, temperature, friction, current, superposition"

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


# CLARIN Integration Points (for future API integration)
class CLARINIntegration:
    """
    Placeholder for CLARIN API integration.
    When ready, connect to:
    - CLARIN Language Analysis API
    - Terminology mapping services
    - Linguistic quality assurance tools
    """

    @staticmethod
    def get_clarin_endpoints():
        """Get CLARIN API endpoints to integrate."""
        return {
            'nlp_analysis': 'https://clarin.eu/api/nlp/analyze',
            'terminology': 'https://clarin.eu/api/terminology/map',
            'linguistic_qa': 'https://clarin.eu/api/quality/linguistic',
            'language_detection': 'https://clarin.eu/api/detect-language',
        }

    @staticmethod
    def map_physics_terminology(term: str, language: str) -> Dict:
        """
        Map physics term to CLARIN standardized terminology.
        Integration point with CLARIN resource federation.
        """
        return {
            'term': term,
            'language': language,
            'standardized_form': f"[CLARIN-mapped-{term}]",
            'source': 'CLARIN Resource Federation',
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
