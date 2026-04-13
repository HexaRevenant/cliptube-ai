const repoUrl = 'https://github.com/HexaRevenant/cliptube-ai';
const releasesUrl = `${repoUrl}/releases/latest`;
const licenseUrl = `${repoUrl}/blob/main/LICENSE`;

const translations = {
  en: {
    navFeatures: 'Features',
    navWorkflow: 'How it works',
    navDownloads: 'Downloads',
    navWebApp: 'Web App',
    navFaq: 'FAQ',
    heroEyebrow: 'Native desktop · Rust + Ollama · Linux / macOS / Windows',
    heroTitle: 'Turn long YouTube videos into reusable intelligence.',
    heroText: 'ClipTube AI extracts transcripts, builds summaries, surfaces key points, gives you a contextual AI chat, and turns video knowledge into text you can paste, refine and share fast.',
    downloadBtn: 'Download',
    webAppBtn: 'Open web app', localRuntimeBtn: 'Open local runtime',
    githubBtn: 'View on GitHub',
    heroMeta1: 'Built for creators', heroMeta2: 'Students', heroMeta3: 'Researchers', heroMeta4: 'Productivity nerds',
    heroBadge1: 'AI chat', heroBadge2: 'Ready to share',
    metric1Title: 'Full transcript', metric1Text: 'Pull the video text and work from the source.',
    metric2Title: 'Ollama-native', metric2Text: 'Choose local models directly from your installed catalog.',
    metric3Title: 'Cross-platform', metric3Text: 'Desktop release flow prepared for Linux, macOS and Windows.',
    featuresEyebrow: 'Core capabilities',
    featuresTitle: 'Everything you need to go from “I should watch that later” to “I already extracted the value.”',
    feature1Title: 'Transcript first', feature1Text: 'Paste a YouTube URL or ID and recover the full transcript as the source of truth.',
    feature2Title: 'AI summary', feature2Text: 'Generate clean summaries optimized for understanding, scanning and reuse.',
    feature3Title: 'Key points', feature3Text: 'Extract the ideas worth remembering without hunting through the full video.',
    feature4Title: 'Contextual AI chat', feature4Text: 'Ask direct questions about the video and refine outputs without leaving the app.',
    feature5Title: 'Ready-to-share text', feature5Text: 'Produce polished text blocks you can paste into chats, notes, docs or posts.',
    feature6Title: 'Live Ollama model selector', feature6Text: 'Load available models straight from Ollama and swap them from the UI.',
    feature7Title: 'Multilingual aware', feature7Text: 'System-language detection and multilingual support make the app friendlier out of the box.',
    feature8Title: 'Native Rust desktop', feature8Text: 'Built with Rust + eframe/egui for a fast, local-first desktop workflow.',
    workflowEyebrow: 'How it works', workflowTitle: 'Fast enough for daily use. Structured enough for real work.',
    step1Title: 'Paste the video', step1Text: 'Drop a YouTube URL or video ID into ClipTube AI.',
    step2Title: 'Fetch the transcript', step2Text: 'Pull the raw transcript and prepare the source material for analysis.',
    step3Title: 'Generate useful output', step3Text: 'Create summaries, key points, AI answers and share-ready text blocks.',
    step4Title: 'Copy, refine, publish', step4Text: 'Reuse the result in messages, docs, notes, research workflows or content pipelines.',
    shareEyebrow: 'Ready-to-share text',
    shareTitle: 'Copy the output and drop it straight into chat, docs or notes.',
    shareText: 'ClipTube AI is designed to turn video knowledge into reusable text. This example block shows the kind of output the app helps you produce once the transcript, summary and key points are ready.',
    shareExampleTitle: 'Example output', copyBtn: 'Copy text', copySuccess: 'Text copied to clipboard.',
    shareExample: `Summary\n\nVideo about rapid knowledge extraction from long-form YouTube content. Main focus: turn transcript-heavy videos into short, reusable insights fast.\n\nKey Points\n- Pull the transcript first so the summary stays grounded in the original content.\n- Extract key points that can be reused in notes, research or team updates.\n- Generate ready-to-share text so the result is useful immediately.\n\nURL\nhttps://www.youtube.com/watch?v=example`,
    downloadsEyebrow: 'Platform support', downloadsTitle: 'Download the build that fits your desktop.',
    webLabel: '🌐 Web', webTitle: 'Browser app', webText: 'Open the web runtime with Ollama connection, transcript backend support and the same core flow as the desktop app.', webBtn: 'Open web app',
    linuxLabel: '🐧 Linux', linuxTitle: 'AppImage', linuxText: 'Portable Linux build with desktop integration support.', linuxBtn: 'Get Linux build',
    macLabel: '🍎 macOS', macTitle: 'DMG + App bundle', macText: 'Drag-and-drop installation flow plus zipped `.app` bundle for releases.', macBtn: 'Get macOS build',
    winLabel: '🪟 Windows', winTitle: 'ZIP / EXE', winText: 'Native executable package with Windows icon resources included.', winBtn: 'Get Windows build',
    previewEyebrow: 'Preview', previewTitle: 'Built for serious note-taking, summarization and content reuse.', previewText: 'The preview below is a polished placeholder composition until final product screenshots are added. The section is already structured so you can swap in real captures later without changing the page layout.',
    ossEyebrow: 'Open source', ossTitle: 'Local-first workflow. Native desktop feel. Transparent stack.',
    oss1Title: 'Rust + eframe/egui', oss1Text: 'Native desktop experience with a modern Rust stack.',
    oss2Title: 'Ollama integration', oss2Text: 'Use your local model catalog instead of locking everything behind a hosted SaaS.',
    oss3Title: 'GitHub-native', oss3Text: 'Releases, packaging, icons and automation are all versioned in the repo.',
    ossCtaText: 'Want to inspect the code, follow releases, or contribute?', ossCtaBtn: 'Explore the repository',
    faqEyebrow: 'FAQ', faqTitle: 'Quick answers before you download.',
    faq1Q: 'Does ClipTube AI need internet?', faq1A: 'You need internet to access YouTube content. The AI model side is designed around Ollama, so summarization and chat can run with your local setup.',
    faq2Q: 'Does it work with Ollama locally?', faq2A: 'Yes. ClipTube AI can list models directly from your local Ollama instance and use them for summaries and chat.',
    faq3Q: 'Is it available for Linux, macOS and Windows?', faq3A: 'Yes. The project is prepared to publish Linux AppImage, macOS DMG / app bundle, and Windows ZIP / EXE packages.',
    faq4Q: 'Is it open source?', faq4A: 'Yes. The repository, release automation, packaging and assets are meant to be shared publicly through GitHub.',
    footerText: 'Turn videos into transcript-powered summaries, insights and shareable output.',
    footerGithub: 'GitHub', footerReleases: 'Releases', footerLicense: 'License', footerNote: 'ClipTube AI. Built for fast understanding and reuse.'
  },
  es: {
    navFeatures: 'Funciones', navWorkflow: 'Cómo funciona', navDownloads: 'Descargas', navWebApp: 'App web', navFaq: 'FAQ',
    heroEyebrow: 'App nativa · Rust + Ollama · Linux / macOS / Windows',
    heroTitle: 'Convierte videos largos de YouTube en inteligencia reutilizable.',
    heroText: 'ClipTube AI extrae transcripciones, crea resúmenes, detecta puntos clave, te da un chat contextual con IA y transforma el conocimiento del video en texto listo para copiar, refinar y compartir.',
    downloadBtn: 'Descargar', webAppBtn: 'Abrir app web', localRuntimeBtn: 'Abrir runtime local', githubBtn: 'Ver en GitHub',
    heroMeta1: 'Pensado para creadores', heroMeta2: 'Estudiantes', heroMeta3: 'Investigadores', heroMeta4: 'Fans de la productividad',
    heroBadge1: 'Chat IA', heroBadge2: 'Listo para compartir',
    metric1Title: 'Transcripción completa', metric1Text: 'Obtén el texto del video y trabaja desde la fuente.',
    metric2Title: 'Ollama nativo', metric2Text: 'Elige modelos locales directamente desde tu catálogo instalado.',
    metric3Title: 'Multiplataforma', metric3Text: 'Flujo de releases preparado para Linux, macOS y Windows.',
    featuresEyebrow: 'Capacidades clave',
    featuresTitle: 'Todo lo que necesitas para pasar de “lo veo después” a “ya extraje el valor”.',
    feature1Title: 'Primero la transcripción', feature1Text: 'Pega una URL o ID de YouTube y recupera la transcripción completa como fuente de verdad.',
    feature2Title: 'Resumen con IA', feature2Text: 'Genera resúmenes limpios y útiles para entender, escanear y reutilizar.',
    feature3Title: 'Puntos importantes', feature3Text: 'Extrae las ideas que vale la pena recordar sin revisar todo el video.',
    feature4Title: 'Chat contextual con IA', feature4Text: 'Haz preguntas directas sobre el video y refina salidas sin salir de la app.',
    feature5Title: 'Texto listo para pegar', feature5Text: 'Produce bloques pulidos para chats, notas, documentos o posts.',
    feature6Title: 'Selector de modelos Ollama', feature6Text: 'Carga modelos disponibles desde Ollama y cámbialos desde la interfaz.',
    feature7Title: 'Multiidioma', feature7Text: 'La detección del idioma del sistema y el soporte multilenguaje mejoran la experiencia desde el inicio.',
    feature8Title: 'Desktop nativo en Rust', feature8Text: 'Construido con Rust + eframe/egui para un flujo local y rápido.',
    workflowEyebrow: 'Cómo funciona', workflowTitle: 'Lo bastante rápido para uso diario. Lo bastante estructurado para trabajo real.',
    step1Title: 'Pega el video', step1Text: 'Ingresa una URL o ID de YouTube en ClipTube AI.',
    step2Title: 'Obtén la transcripción', step2Text: 'Trae el transcript bruto y prepara el material para analizarlo.',
    step3Title: 'Genera contenido útil', step3Text: 'Crea resúmenes, puntos clave, respuestas con IA y texto listo para compartir.',
    step4Title: 'Copia, refina y publica', step4Text: 'Reutiliza el resultado en mensajes, docs, notas o flujos de investigación.',
    shareEyebrow: 'Texto listo para compartir', shareTitle: 'Copia la salida y llévala directo a chat, documentos o notas.',
    shareText: 'ClipTube AI está pensado para convertir el conocimiento del video en texto reutilizable. Este bloque muestra el tipo de salida que puedes obtener cuando ya tienes transcript, resumen y puntos clave.',
    shareExampleTitle: 'Ejemplo de salida', copyBtn: 'Copiar texto', copySuccess: 'Texto copiado al portapapeles.',
    shareExample: `Resumen\n\nVideo sobre extracción rápida de conocimiento desde contenido largo de YouTube. Enfoque principal: convertir videos con mucha transcripción en ideas cortas y reutilizables.\n\nPuntos Importantes\n- Obtener primero la transcripción para que el resumen se mantenga fiel a la fuente.\n- Extraer puntos clave que puedan reutilizarse en notas, investigación o actualizaciones de equipo.\n- Generar texto listo para compartir para que el resultado sea útil de inmediato.\n\nURL\nhttps://www.youtube.com/watch?v=example`,
    downloadsEyebrow: 'Soporte por plataforma', downloadsTitle: 'Descarga la build que mejor calza con tu escritorio.',
    webLabel: '🌐 Web', webTitle: 'App web', webText: 'Abre la versión web con conexión a Ollama, soporte de backend para transcript y el mismo flujo central que la app de escritorio.', webBtn: 'Abrir app web',
    linuxLabel: '🐧 Linux', linuxTitle: 'AppImage', linuxText: 'Build portable para Linux con soporte de integración de escritorio.', linuxBtn: 'Descargar Linux',
    macLabel: '🍎 macOS', macTitle: 'DMG + App bundle', macText: 'Instalación drag-and-drop más bundle `.app` comprimido para releases.', macBtn: 'Descargar macOS',
    winLabel: '🪟 Windows', winTitle: 'ZIP / EXE', winText: 'Paquete ejecutable nativo con recursos de icono para Windows.', winBtn: 'Descargar Windows',
    previewEyebrow: 'Vista previa', previewTitle: 'Hecho para notas serias, resúmenes y reutilización de contenido.', previewText: 'La vista inferior es un placeholder pulido hasta que agreguemos capturas reales. La sección ya está lista para cambiarlas sin romper el layout.',
    ossEyebrow: 'Código abierto', ossTitle: 'Flujo local primero. Sensación desktop nativa. Stack transparente.',
    oss1Title: 'Rust + eframe/egui', oss1Text: 'Experiencia desktop nativa con un stack moderno en Rust.',
    oss2Title: 'Integración con Ollama', oss2Text: 'Usa tu catálogo local de modelos en vez de depender de un SaaS cerrado.',
    oss3Title: 'GitHub-native', oss3Text: 'Releases, packaging, iconos y automatización versionados en el repo.',
    ossCtaText: '¿Quieres inspeccionar el código, seguir releases o contribuir?', ossCtaBtn: 'Explorar el repositorio',
    faqEyebrow: 'FAQ', faqTitle: 'Respuestas rápidas antes de descargar.',
    faq1Q: '¿ClipTube AI necesita internet?', faq1A: 'Necesitas internet para acceder al contenido de YouTube. La parte de IA está pensada para Ollama, así que el resumen y chat pueden correr con tu setup local.',
    faq2Q: '¿Funciona con Ollama local?', faq2A: 'Sí. ClipTube AI puede listar modelos desde tu instancia local de Ollama y usarlos para resúmenes y chat.',
    faq3Q: '¿Está disponible para Linux, macOS y Windows?', faq3A: 'Sí. El proyecto está preparado para publicar AppImage en Linux, DMG / app bundle en macOS y ZIP / EXE en Windows.',
    faq4Q: '¿Es open source?', faq4A: 'Sí. El repositorio, la automatización de releases, el packaging y los assets están pensados para compartirse públicamente en GitHub.',
    footerText: 'Convierte videos en resúmenes, insights y texto listo para compartir.', footerGithub: 'GitHub', footerReleases: 'Releases', footerLicense: 'Licencia', footerNote: 'ClipTube AI. Hecho para entender y reutilizar más rápido.'
  },
  pt: {
    navFeatures: 'Recursos', navWorkflow: 'Como funciona', navDownloads: 'Downloads', navWebApp: 'App web', navFaq: 'FAQ',
    heroEyebrow: 'Desktop nativo · Rust + Ollama · Linux / macOS / Windows',
    heroTitle: 'Transforme vídeos longos do YouTube em inteligência reutilizável.',
    heroText: 'ClipTube AI extrai transcrições, cria resumos, destaca pontos-chave, oferece chat contextual com IA e transforma conhecimento em texto pronto para copiar, refinar e compartilhar.',
    downloadBtn: 'Baixar', webAppBtn: 'Abrir app web', githubBtn: 'Ver no GitHub',
    heroMeta1: 'Feito para creators', heroMeta2: 'Estudantes', heroMeta3: 'Pesquisadores', heroMeta4: 'Fanáticos por produtividade',
    heroBadge1: 'Chat IA', heroBadge2: 'Pronto para compartilhar',
    metric1Title: 'Transcrição completa', metric1Text: 'Puxe o texto do vídeo e trabalhe a partir da fonte.',
    metric2Title: 'Ollama nativo', metric2Text: 'Escolha modelos locais diretamente do seu catálogo instalado.',
    metric3Title: 'Multiplataforma', metric3Text: 'Fluxo de releases preparado para Linux, macOS e Windows.',
    featuresEyebrow: 'Capacidades principais',
    featuresTitle: 'Tudo o que você precisa para sair do “vejo isso depois” para “já extraí o valor”.',
    feature1Title: 'Transcrição primeiro', feature1Text: 'Cole uma URL ou ID do YouTube e recupere a transcrição completa como fonte de verdade.',
    feature2Title: 'Resumo com IA', feature2Text: 'Gere resumos limpos e úteis para entender, escanear e reutilizar.',
    feature3Title: 'Pontos importantes', feature3Text: 'Extraia as ideias que valem ser lembradas sem rever o vídeo inteiro.',
    feature4Title: 'Chat contextual com IA', feature4Text: 'Faça perguntas diretas sobre o vídeo e refine saídas sem sair do app.',
    feature5Title: 'Texto pronto para colar', feature5Text: 'Produza blocos polidos para chats, notas, documentos ou posts.',
    feature6Title: 'Seletor de modelos Ollama', feature6Text: 'Carregue modelos disponíveis diretamente do Ollama e troque pela interface.',
    feature7Title: 'Multilíngue', feature7Text: 'Detecção do idioma do sistema e suporte multilíngue melhoram a experiência desde o início.',
    feature8Title: 'Desktop nativo em Rust', feature8Text: 'Construído com Rust + eframe/egui para um fluxo local e rápido.',
    workflowEyebrow: 'Como funciona', workflowTitle: 'Rápido o bastante para o dia a dia. Estruturado o suficiente para trabalho real.',
    step1Title: 'Cole o vídeo', step1Text: 'Insira uma URL ou ID do YouTube no ClipTube AI.',
    step2Title: 'Obtenha a transcrição', step2Text: 'Busque a transcrição bruta e prepare o material para análise.',
    step3Title: 'Gere saída útil', step3Text: 'Crie resumos, pontos-chave, respostas com IA e texto pronto para compartilhar.',
    step4Title: 'Copie, refine e publique', step4Text: 'Reaproveite o resultado em mensagens, docs, notas ou fluxos de pesquisa.',
    shareEyebrow: 'Texto pronto para compartilhar', shareTitle: 'Copie a saída e leve direto para chat, documentos ou notas.',
    shareText: 'ClipTube AI foi feito para transformar o conhecimento do vídeo em texto reutilizável. Este bloco mostra o tipo de saída que o app pode gerar quando a transcrição, o resumo e os pontos-chave já estão prontos.',
    shareExampleTitle: 'Exemplo de saída', copyBtn: 'Copiar texto', copySuccess: 'Texto copiado para a área de transferência.',
    shareExample: `Resumo\n\nVídeo sobre extração rápida de conhecimento de conteúdo longo do YouTube. Foco principal: transformar vídeos com muita transcrição em ideias curtas e reutilizáveis.\n\nPontos Importantes\n- Buscar primeiro a transcrição para que o resumo permaneça fiel à fonte.\n- Extrair pontos-chave que possam ser reutilizados em notas, pesquisa ou atualizações de equipe.\n- Gerar texto pronto para compartilhar para que o resultado seja útil imediatamente.\n\nURL\nhttps://www.youtube.com/watch?v=example`,
    downloadsEyebrow: 'Suporte por plataforma', downloadsTitle: 'Baixe a build ideal para o seu desktop.',
    webLabel: '🌐 Web', webTitle: 'App web', webText: 'Abra a versão web com conexão ao Ollama, suporte a backend de transcrição e o mesmo fluxo principal da app desktop.', webBtn: 'Abrir app web',
    linuxLabel: '🐧 Linux', linuxTitle: 'AppImage', linuxText: 'Build portátil para Linux com suporte de integração ao desktop.', linuxBtn: 'Baixar Linux',
    macLabel: '🍎 macOS', macTitle: 'DMG + App bundle', macText: 'Instalação drag-and-drop mais bundle `.app` compactado para releases.', macBtn: 'Baixar macOS',
    winLabel: '🪟 Windows', winTitle: 'ZIP / EXE', winText: 'Pacote executável nativo com recursos de ícone para Windows.', winBtn: 'Baixar Windows',
    previewEyebrow: 'Prévia', previewTitle: 'Feito para anotações sérias, resumos e reutilização de conteúdo.', previewText: 'A prévia abaixo é um placeholder elegante até adicionarmos capturas reais. A seção já está pronta para trocar as imagens sem quebrar o layout.',
    ossEyebrow: 'Open source', ossTitle: 'Fluxo local-first. Sensação desktop nativa. Stack transparente.',
    oss1Title: 'Rust + eframe/egui', oss1Text: 'Experiência desktop nativa com um stack moderno em Rust.',
    oss2Title: 'Integração com Ollama', oss2Text: 'Use seu catálogo local de modelos em vez de depender de um SaaS fechado.',
    oss3Title: 'GitHub-native', oss3Text: 'Releases, packaging, ícones e automação versionados no repositório.',
    ossCtaText: 'Quer inspecionar o código, acompanhar releases ou contribuir?', ossCtaBtn: 'Explorar o repositório',
    faqEyebrow: 'FAQ', faqTitle: 'Respostas rápidas antes de baixar.',
    faq1Q: 'O ClipTube AI precisa de internet?', faq1A: 'Você precisa de internet para acessar o conteúdo do YouTube. A parte de IA é pensada para Ollama, então resumo e chat podem rodar com sua configuração local.',
    faq2Q: 'Funciona com Ollama local?', faq2A: 'Sim. O ClipTube AI pode listar modelos diretamente da sua instância local do Ollama e usá-los em resumos e chat.',
    faq3Q: 'Está disponível para Linux, macOS e Windows?', faq3A: 'Sim. O projeto está preparado para publicar AppImage no Linux, DMG / app bundle no macOS e ZIP / EXE no Windows.',
    faq4Q: 'É open source?', faq4A: 'Sim. O repositório, a automação de releases, o packaging e os assets foram pensados para serem compartilhados publicamente no GitHub.',
    footerText: 'Transforme vídeos em resumos, insights e texto pronto para compartilhar.', footerGithub: 'GitHub', footerReleases: 'Releases', footerLicense: 'Licença', footerNote: 'ClipTube AI. Feito para entender e reutilizar mais rápido.'
  }
};

translations.fr = {
  ...translations.en,
  navFeatures: 'Fonctionnalités', navWorkflow: 'Comment ça marche', navDownloads: 'Téléchargements',
  heroEyebrow: 'Desktop natif · Rust + Ollama · Linux / macOS / Windows',
  heroTitle: 'Transformez de longues vidéos YouTube en intelligence réutilisable.',
  heroText: 'ClipTube AI extrait les transcriptions, crée des résumés, repère les points clés, ajoute un chat IA contextuel et transforme la connaissance vidéo en texte prêt à copier, affiner et partager.',
  downloadBtn: 'Télécharger', webAppBtn: 'Ouvrir l’app web', githubBtn: 'Voir sur GitHub',
  heroMeta1: 'Pensé pour les créateurs', heroMeta2: 'Étudiants', heroMeta3: 'Chercheurs', heroMeta4: 'Obsédés de productivité',
  heroBadge1: 'Chat IA', heroBadge2: 'Prêt à partager',
  metric1Title: 'Transcription complète', metric1Text: 'Récupérez le texte de la vidéo et travaillez depuis la source.',
  metric2Title: 'Ollama natif', metric2Text: 'Choisissez vos modèles locaux directement depuis votre catalogue installé.',
  metric3Title: 'Multiplateforme', metric3Text: 'Flux de release préparé pour Linux, macOS et Windows.',
  featuresEyebrow: 'Capacités clés', featuresTitle: 'Tout ce qu’il faut pour passer de « je regarderai plus tard » à « j’ai déjà extrait la valeur ».', feature1Title: 'Transcription d’abord', feature1Text: 'Collez une URL ou un ID YouTube et récupérez la transcription complète comme source de vérité.', feature2Title: 'Résumé IA', feature2Text: 'Générez des résumés propres et utiles pour comprendre, scanner et réutiliser.', feature3Title: 'Points clés', feature3Text: 'Extrayez les idées à retenir sans revoir toute la vidéo.', feature4Title: 'Chat IA contextuel', feature4Text: 'Posez des questions directes sur la vidéo et affinez les sorties sans quitter l’app.', feature5Title: 'Texte prêt à coller', feature5Text: 'Produisez des blocs propres pour chats, notes, documents ou posts.', feature6Title: 'Sélecteur de modèles Ollama', feature6Text: 'Chargez les modèles disponibles depuis Ollama et changez-les depuis l’interface.', feature7Title: 'Multilingue', feature7Text: 'La détection de langue système et le support multilingue améliorent l’expérience dès le départ.', feature8Title: 'Desktop natif en Rust', feature8Text: 'Construit avec Rust + eframe/egui pour un flux local, rapide et fiable.',
  workflowEyebrow: 'Comment ça marche', workflowTitle: 'Assez rapide pour tous les jours. Assez structuré pour un vrai travail.', step1Title: 'Collez la vidéo', step1Text: 'Ajoutez une URL ou un ID YouTube dans ClipTube AI.', step2Title: 'Récupérez la transcription', step2Text: 'Récupérez la transcription brute et préparez la matière pour l’analyse.', step3Title: 'Générez une sortie utile', step3Text: 'Créez des résumés, points clés, réponses IA et blocs prêts à partager.', step4Title: 'Copiez, affinez, publiez', step4Text: 'Réutilisez le résultat dans des messages, docs, notes ou workflows de recherche.',
  shareEyebrow: 'Texte prêt à partager', shareTitle: 'Copiez la sortie et collez-la directement dans vos chats, documents ou notes.', shareText: 'ClipTube AI est conçu pour transformer la connaissance d’une vidéo en texte réutilisable. Ce bloc montre le type de sortie que l’application peut produire une fois la transcription, le résumé et les points clés prêts.', shareExampleTitle: 'Exemple de sortie', copyBtn: 'Copier le texte', copySuccess: 'Texte copié dans le presse-papiers.',
  shareExample: `Résumé\n\nVidéo sur l’extraction rapide de connaissance à partir de contenus YouTube longs. Objectif principal : transformer les vidéos riches en transcript en idées courtes et réutilisables.\n\nPoints importants\n- Récupérer d’abord la transcription pour que le résumé reste fidèle à la source.\n- Extraire des points clés réutilisables dans des notes, recherches ou mises à jour d’équipe.\n- Générer du texte prêt à partager pour rendre le résultat immédiatement utile.\n\nURL\nhttps://www.youtube.com/watch?v=example`,
  downloadsEyebrow: 'Support plateforme', downloadsTitle: 'Téléchargez la build adaptée à votre desktop.', linuxText: 'Build portable Linux avec prise en charge de l’intégration au bureau.', linuxBtn: 'Télécharger Linux', macText: 'Installation glisser-déposer avec bundle `.app` zippé pour les releases.', macBtn: 'Télécharger macOS', winText: 'Paquet natif avec ressources d’icône Windows incluses.', winBtn: 'Télécharger Windows',
  previewEyebrow: 'Aperçu', previewTitle: 'Pensé pour la prise de notes sérieuse, les résumés et la réutilisation de contenu.', previewText: 'L’aperçu ci-dessous est un placeholder élégant jusqu’à l’ajout de captures réelles. La section est déjà prête à les remplacer sans casser la mise en page.',
  ossEyebrow: 'Open source', ossTitle: 'Workflow local-first. Sensation desktop native. Stack transparent.', oss1Text: 'Expérience desktop native avec une stack Rust moderne.', oss2Text: 'Utilisez votre catalogue local au lieu d’un SaaS fermé.', oss3Text: 'Releases, packaging, icônes et automatisation versionnés dans le dépôt.', ossCtaText: 'Vous voulez inspecter le code, suivre les releases ou contribuer ?', ossCtaBtn: 'Explorer le dépôt',
  faqTitle: 'Réponses rapides avant de télécharger.', faq1Q: 'ClipTube AI a-t-il besoin d’internet ?', faq1A: 'Vous avez besoin d’internet pour accéder au contenu YouTube. La partie IA est pensée pour Ollama, donc résumés et chat peuvent tourner avec votre setup local.', faq2Q: 'Fonctionne-t-il avec Ollama en local ?', faq2A: 'Oui. ClipTube AI peut lister les modèles de votre instance locale d’Ollama et les utiliser pour les résumés et le chat.', faq3Q: 'Disponible sur Linux, macOS et Windows ?', faq3A: 'Oui. Le projet est prêt pour publier un AppImage Linux, un DMG / bundle `.app` macOS et des ZIP / EXE Windows.', faq4Q: 'Est-ce open source ?', faq4A: 'Oui. Le dépôt, l’automatisation des releases, le packaging et les assets sont pensés pour être partagés publiquement sur GitHub.',
  footerText: 'Transformez les vidéos en résumés, insights et texte prêt à partager.', footerLicense: 'Licence', footerNote: 'ClipTube AI. Conçu pour comprendre et réutiliser plus vite.'
};
translations.de = {
  ...translations.en,
  navFeatures: 'Funktionen', navWorkflow: 'So funktioniert es', navDownloads: 'Downloads',
  heroTitle: 'Verwandle lange YouTube-Videos in wiederverwendbare Erkenntnisse.',
  heroText: 'ClipTube AI extrahiert Transkripte, erstellt Zusammenfassungen, erkennt Kernpunkte, bietet einen kontextbezogenen KI-Chat und verwandelt Videowissen in Text zum Kopieren, Verfeinern und Teilen.',
  downloadBtn: 'Herunterladen', webAppBtn: 'Web-App öffnen', githubBtn: 'Auf GitHub ansehen', heroMeta1: 'Für Creators', heroMeta2: 'Studierende', heroMeta3: 'Forscher', heroMeta4: 'Produktivitätsfans',
  heroBadge1: 'KI-Chat', heroBadge2: 'Bereit zum Teilen',
  metric1Title: 'Vollständiges Transkript', metric1Text: 'Hole den Videotext und arbeite direkt von der Quelle aus.',
  metric2Title: 'Ollama-nativ', metric2Text: 'Wähle lokale Modelle direkt aus deinem installierten Katalog.',
  metric3Title: 'Plattformübergreifend', metric3Text: 'Release-Workflow vorbereitet für Linux, macOS und Windows.',
  featuresEyebrow: 'Kernfunktionen', featuresTitle: 'Alles, was du brauchst, um von „später ansehen“ zu „Wert bereits extrahiert“ zu kommen.', feature1Title: 'Transkript zuerst', feature1Text: 'Füge eine YouTube-URL oder -ID ein und hole das vollständige Transkript als zuverlässige Quelle.', feature2Title: 'KI-Zusammenfassung', feature2Text: 'Erzeuge saubere, nützliche Zusammenfassungen zum Verstehen, Scannen und Wiederverwenden.', feature3Title: 'Wichtige Punkte', feature3Text: 'Extrahiere die wichtigsten Ideen, ohne das ganze Video erneut zu sehen.', feature4Title: 'Kontextbezogener KI-Chat', feature4Text: 'Stelle direkte Fragen zum Video und verbessere Ausgaben, ohne die App zu verlassen.', feature5Title: 'Text zum Einfügen', feature5Text: 'Erzeuge polierte Blöcke für Chats, Notizen, Dokumente oder Posts.', feature6Title: 'Ollama-Modellwahlschalter', feature6Text: 'Lade verfügbare Modelle direkt aus Ollama und wechsle sie über die Oberfläche.', feature7Title: 'Mehrsprachig', feature7Text: 'Systemsprachenerkennung und Mehrsprachigkeit verbessern die Erfahrung von Anfang an.', feature8Title: 'Native Rust-Desktop-App', feature8Text: 'Gebaut mit Rust + eframe/egui für einen schnellen, lokalen Workflow.',
  workflowEyebrow: 'So funktioniert es', workflowTitle: 'Schnell genug für den Alltag. Strukturiert genug für echte Arbeit.', step1Title: 'Video einfügen', step1Text: 'Füge eine YouTube-URL oder Video-ID in ClipTube AI ein.', step2Title: 'Transkript holen', step2Text: 'Hole das Rohtranskript und bereite das Material für die Analyse vor.', step3Title: 'Nützliche Ausgabe erzeugen', step3Text: 'Erstelle Zusammenfassungen, Schlüsselideen, KI-Antworten und teilbare Textblöcke.', step4Title: 'Kopieren, verfeinern, veröffentlichen', step4Text: 'Verwende das Ergebnis in Nachrichten, Docs, Notizen oder Research-Workflows weiter.',
  shareEyebrow: 'Teilbarer Text', shareTitle: 'Kopiere die Ausgabe und füge sie direkt in Chat, Dokumente oder Notizen ein.', shareText: 'ClipTube AI wurde dafür entwickelt, Videowissen in wiederverwendbaren Text zu verwandeln. Dieser Beispielblock zeigt die Art von Ausgabe, die die App erzeugen kann, sobald Transkript, Zusammenfassung und Kernpunkte bereit sind.', shareExampleTitle: 'Beispielausgabe', copyBtn: 'Text kopieren', copySuccess: 'Text in die Zwischenablage kopiert.',
  shareExample: `Zusammenfassung\n\nVideo über schnelle Wissensextraktion aus langen YouTube-Inhalten. Hauptziel: transcriptlastige Videos in kurze, wiederverwendbare Erkenntnisse verwandeln.\n\nWichtige Punkte\n- Zuerst das Transkript holen, damit die Zusammenfassung an der Quelle bleibt.\n- Schlüsselpunkte extrahieren, die in Notizen, Forschung oder Team-Updates wiederverwendet werden können.\n- Sofort teilbaren Text erzeugen, damit das Ergebnis direkt nutzbar ist.\n\nURL\nhttps://www.youtube.com/watch?v=example`,
  downloadsEyebrow: 'Plattformsupport', downloadsTitle: 'Lade die Build herunter, die zu deinem Desktop passt.',
  previewEyebrow: 'Vorschau', previewTitle: 'Gebaut für ernsthafte Notizen, Zusammenfassungen und die Wiederverwendung von Inhalten.',
  ossEyebrow: 'Open Source', ossTitle: 'Local-first-Workflow. Native Desktop-Erfahrung. Transparenter Stack.',
  faqTitle: 'Schnelle Antworten vor dem Download.', footerText: 'Verwandle Videos in transkriptgestützte Zusammenfassungen, Insights und teilbaren Output.', footerNote: 'ClipTube AI. Gebaut für schnelleres Verstehen und Wiederverwenden.'
};
translations.ja = {
  ...translations.en,
  navFeatures: '機能', navWorkflow: '使い方', navDownloads: 'ダウンロード',
  heroTitle: '長い YouTube 動画を再利用できる知識に変えよう。',
  heroText: 'ClipTube AI は文字起こしを取得し、要約を作成し、重要ポイントを抽出し、文脈付き AI チャットを提供し、動画の知識をコピー・改善・共有しやすいテキストに変換します。',
  downloadBtn: 'ダウンロード', webAppBtn: 'Webアプリを開く', githubBtn: 'GitHubで見る', heroMeta1: 'クリエイター向け', heroMeta2: '学生向け', heroMeta3: '研究者向け', heroMeta4: '生産性好き向け',
  heroBadge1: 'AI チャット', heroBadge2: 'すぐ共有',
  metric1Title: '完全な文字起こし', metric1Text: '動画テキストを取得し、ソースから作業できます。',
  metric2Title: 'Ollama ネイティブ', metric2Text: 'インストール済みのローカルモデルを直接選択できます。',
  metric3Title: 'クロスプラットフォーム', metric3Text: 'Linux / macOS / Windows 向けのリリースフローを用意。',
  featuresEyebrow: '主な機能', featuresTitle: '「あとで見よう」から「価値をもう抽出した」へ進むために必要なすべて。', feature1Title: 'まず文字起こし', feature1Text: 'YouTube URL または ID を貼り付け、全文文字起こしを信頼できるソースとして取得します。', feature2Title: 'AI 要約', feature2Text: '理解・確認・再利用に向いたクリーンな要約を生成します。', feature3Title: '重要ポイント', feature3Text: '動画全体を見返さずに覚えるべき要点を抽出します。', feature4Title: '文脈付き AI チャット', feature4Text: '動画について直接質問し、アプリを離れずに出力を改善できます。', feature5Title: '貼り付け用テキスト', feature5Text: 'チャット、ノート、ドキュメント、投稿向けの整ったテキストを生成します。', feature6Title: 'Ollama モデルセレクター', feature6Text: 'Ollama から利用可能なモデルを読み込み、UI から切り替えられます。', feature7Title: '多言語対応', feature7Text: 'システム言語の検出と多言語サポートで最初から使いやすくなります。', feature8Title: 'Rust ネイティブデスクトップ', feature8Text: 'Rust + eframe/egui で構築された高速なローカルワークフローです。',
  workflowEyebrow: '使い方', workflowTitle: '日常使いに十分な速さ。実務に十分な構造。', step1Title: '動画を貼り付け', step1Text: 'ClipTube AI に YouTube URL または動画 ID を入力します。', step2Title: '文字起こしを取得', step2Text: '生の文字起こしを取得し、分析用の素材を準備します。', step3Title: '役立つ出力を生成', step3Text: '要約、重要ポイント、AI 回答、共有向けテキストブロックを作成します。', step4Title: 'コピーして改善・公開', step4Text: '結果をメッセージ、ドキュメント、ノート、研究フローに再利用します。',
  shareEyebrow: '共有用テキスト', shareTitle: '出力をコピーして、そのままチャットやドキュメント、ノートに貼り付けられます。', shareText: 'ClipTube AI は、動画の知識を再利用できるテキストに変えるためのツールです。この例は、文字起こし・要約・重要ポイントがそろったときに得られる出力を示しています。', shareExampleTitle: '出力例', copyBtn: 'テキストをコピー', copySuccess: 'テキストをクリップボードにコピーしました。',
  downloadsEyebrow: 'プラットフォーム対応', downloadsTitle: 'あなたのデスクトップに合うビルドをダウンロード。',
  previewEyebrow: 'プレビュー', previewTitle: '本格的なノート、要約、コンテンツ再利用のために設計。',
  ossEyebrow: 'オープンソース', ossTitle: 'ローカルファースト。ネイティブなデスクトップ感。透明なスタック。',
  faqTitle: 'ダウンロード前の簡単な回答。', footerText: '動画を文字起こしベースの要約、洞察、共有可能な出力へ変換します。', footerNote: 'ClipTube AI。すばやい理解と再利用のために。'
};
translations['zh-Hans'] = {
  ...translations.en,
  navFeatures: '功能', navWorkflow: '使用方式', navDownloads: '下载',
  heroTitle: '把冗长的 YouTube 视频变成可复用的知识。',
  heroText: 'ClipTube AI 可以提取转录、生成摘要、整理重点、提供上下文 AI 聊天，并把视频知识变成可复制、可润色、可分享的文本。',
  downloadBtn: '下载', webAppBtn: '打开 Web 应用', githubBtn: '查看 GitHub', heroMeta1: '适合创作者', heroMeta2: '学生', heroMeta3: '研究者', heroMeta4: '效率控',
  heroBadge1: 'AI 聊天', heroBadge2: '随手分享',
  metric1Title: '完整转录', metric1Text: '先拿到视频文本，再从源头开始工作。', metric2Title: '原生 Ollama', metric2Text: '直接从已安装的本地模型目录中选择。', metric3Title: '跨平台', metric3Text: '已为 Linux / macOS / Windows 准备桌面发布流程。',
  featuresEyebrow: '核心能力', featuresTitle: '从“以后再看”到“价值已经提取完了”所需的一切。', feature1Title: '先拿转录', feature1Text: '粘贴 YouTube 链接或 ID，获取完整转录作为可信来源。', feature2Title: 'AI 摘要', feature2Text: '生成干净、易扫读、易复用的摘要。', feature3Title: '关键点', feature3Text: '无需重看整段视频，也能提炼值得记住的要点。', feature4Title: '上下文 AI 聊天', feature4Text: '围绕视频直接提问，并在应用内继续打磨输出。', feature5Title: '可直接粘贴的文本', feature5Text: '生成适合聊天、笔记、文档或帖子使用的整洁文本块。', feature6Title: 'Ollama 模型选择器', feature6Text: '直接从 Ollama 加载可用模型，并在界面中切换。', feature7Title: '多语言感知', feature7Text: '系统语言检测和多语言支持让它开箱即用。', feature8Title: 'Rust 原生桌面应用', feature8Text: '基于 Rust + eframe/egui，提供快速、本地优先的工作流。',
  workflowEyebrow: '使用方式', workflowTitle: '足够快，适合日常。足够稳，适合认真工作。', step1Title: '粘贴视频', step1Text: '把 YouTube 链接或视频 ID 放进 ClipTube AI。', step2Title: '获取转录', step2Text: '拉取原始转录，并为分析准备素材。', step3Title: '生成有用输出', step3Text: '创建摘要、重点、AI 回答以及可分享的文本块。', step4Title: '复制、润色、发布', step4Text: '把结果复用于消息、文档、笔记或研究流程。',
  shareEyebrow: '可分享文本', shareTitle: '复制输出，直接粘贴到聊天、文档或笔记里。', shareText: 'ClipTube AI 的目标，是把视频知识变成可复用的文本。这个示例展示了当转录、摘要和重点都准备好后，应用能提供怎样的结果。', shareExampleTitle: '输出示例', copyBtn: '复制文本', copySuccess: '文本已复制到剪贴板。',
  downloadsEyebrow: '平台支持', downloadsTitle: '下载适合你桌面的版本。', previewEyebrow: '预览', previewTitle: '为严肃笔记、摘要和内容复用而打造。', ossEyebrow: '开源', ossTitle: '本地优先工作流。原生桌面体验。透明技术栈。', faqTitle: '下载前的快速解答。', footerText: '把视频变成基于转录的摘要、洞察和可分享输出。', footerNote: 'ClipTube AI。为更快理解与复用而生。'
};
translations.ru = {
  ...translations.en,
  navFeatures: 'Возможности', navWorkflow: 'Как это работает', navDownloads: 'Загрузки',
  heroTitle: 'Превращайте длинные видео YouTube в переиспользуемые знания.',
  heroText: 'ClipTube AI извлекает транскрипты, делает сводки, выделяет ключевые моменты, даёт контекстный ИИ-чат и превращает знания из видео в текст, который можно копировать, дорабатывать и отправлять.',
  downloadBtn: 'Скачать', webAppBtn: 'Открыть веб-приложение', githubBtn: 'Открыть GitHub', heroMeta1: 'Для креаторов', heroMeta2: 'Студентов', heroMeta3: 'Исследователей', heroMeta4: 'Фанатов продуктивности',
  heroBadge1: 'ИИ-чат', heroBadge2: 'Готово к отправке',
  metric1Title: 'Полный транскрипт', metric1Text: 'Получите текст видео и работайте от первоисточника.', metric2Title: 'Нативный Ollama', metric2Text: 'Выбирайте локальные модели прямо из установленного каталога.', metric3Title: 'Кроссплатформенность', metric3Text: 'Подготовлен релизный поток для Linux / macOS / Windows.',
  featuresEyebrow: 'Ключевые возможности', featuresTitle: 'Всё, что нужно, чтобы перейти от «посмотрю потом» к «ценность уже извлечена».', feature1Title: 'Сначала транскрипт', feature1Text: 'Вставьте URL или ID YouTube и получите полный транскрипт как источник истины.', feature2Title: 'ИИ-сводка', feature2Text: 'Создавайте чистые и полезные сводки для понимания и повторного использования.', feature3Title: 'Ключевые пункты', feature3Text: 'Извлекайте важные идеи без пересмотра всего видео.', feature4Title: 'Контекстный ИИ-чат', feature4Text: 'Задавайте прямые вопросы о видео и улучшайте результат, не выходя из приложения.', feature5Title: 'Текст для вставки', feature5Text: 'Создавайте аккуратные блоки для чатов, заметок, документов и постов.', feature6Title: 'Выбор моделей Ollama', feature6Text: 'Загружайте доступные модели из Ollama и переключайте их в интерфейсе.', feature7Title: 'Многоязычность', feature7Text: 'Определение языка системы и мультиязычная поддержка делают старт удобнее.', feature8Title: 'Нативный Rust desktop', feature8Text: 'Построено на Rust + eframe/egui для быстрого локального workflow.',
  workflowEyebrow: 'Как это работает', workflowTitle: 'Достаточно быстро для ежедневной работы. Достаточно структурно для серьёзных задач.', shareEyebrow: 'Готовый текст', shareTitle: 'Скопируйте результат и вставьте его сразу в чат, документ или заметку.', copyBtn: 'Скопировать текст', copySuccess: 'Текст скопирован в буфер обмена。',
  downloadsEyebrow: 'Поддержка платформ', downloadsTitle: 'Скачайте сборку под вашу систему.', previewEyebrow: 'Превью', previewTitle: 'Создано для серьёзных заметок, сводок и повторного использования контента.', ossEyebrow: 'Open source', ossTitle: 'Local-first workflow. Нативное desktop-ощущение. Прозрачный стек.', faqTitle: 'Короткие ответы перед загрузкой.', footerText: 'Преобразуйте видео в сводки, инсайты и текст для отправки.', footerNote: 'ClipTube AI. Для быстрого понимания и повторного использования.'
};
translations.ar = {
  ...translations.en,
  navFeatures: 'الميزات', navWorkflow: 'كيف يعمل', navDownloads: 'التنزيلات',
  heroTitle: 'حوّل فيديوهات YouTube الطويلة إلى معرفة قابلة لإعادة الاستخدام.',
  heroText: 'يستخرج ClipTube AI النصوص المفرغة، ويُنشئ الملخصات، ويبرز النقاط المهمة، ويوفر دردشة ذكاء اصطناعي سياقية، ويحوّل معرفة الفيديو إلى نص جاهز للنسخ والتحسين والمشاركة.',
  downloadBtn: 'تنزيل', webAppBtn: 'افتح تطبيق الويب', githubBtn: 'عرض على GitHub', heroMeta1: 'لصنّاع المحتوى', heroMeta2: 'للطلاب', heroMeta3: 'للباحثين', heroMeta4: 'لعشاق الإنتاجية',
  heroBadge1: 'دردشة AI', heroBadge2: 'جاهز للمشاركة',
  metric1Title: 'نص مفرغ كامل', metric1Text: 'احصل على نص الفيديو واعمل من المصدر مباشرة.', metric2Title: 'Ollama محلي', metric2Text: 'اختر النماذج المحلية مباشرة من كتالوجك المثبت.', metric3Title: 'متعدد المنصات', metric3Text: 'سير نشر مكتبي جاهز لـ Linux وmacOS وWindows.',
  featuresEyebrow: 'القدرات الأساسية', featuresTitle: 'كل ما تحتاجه لتنتقل من «سأشاهده لاحقًا» إلى «لقد استخرجت القيمة بالفعل».', feature1Title: 'النص المفرغ أولًا', feature1Text: 'ألصق رابط YouTube أو المعرّف واسترجع النص الكامل كمصدر موثوق.', feature2Title: 'ملخص بالذكاء الاصطناعي', feature2Text: 'أنشئ ملخصات واضحة ومفيدة للفهم والمسح السريع وإعادة الاستخدام.', feature3Title: 'النقاط المهمة', feature3Text: 'استخرج الأفكار التي تستحق التذكر دون إعادة مشاهدة الفيديو بالكامل.', feature4Title: 'دردشة سياقية بالذكاء الاصطناعي', feature4Text: 'اطرح أسئلة مباشرة حول الفيديو وحسّن المخرجات دون مغادرة التطبيق.', feature5Title: 'نص جاهز للصق', feature5Text: 'أنشئ كتلًا مرتبة للدردشة والملاحظات والمستندات أو المنشورات.', feature6Title: 'محدد نماذج Ollama', feature6Text: 'حمّل النماذج المتاحة من Ollama وبدّل بينها من الواجهة.', feature7Title: 'متعدد اللغات', feature7Text: 'اكتشاف لغة النظام والدعم المتعدد اللغات يجعلان البداية أسهل.', feature8Title: 'تطبيق Rust أصلي', feature8Text: 'مبني باستخدام Rust + eframe/egui لتجربة سريعة ومحلية أولًا.',
  workflowEyebrow: 'كيف يعمل', workflowTitle: 'سريع بما يكفي للاستخدام اليومي. منظم بما يكفي للعمل الحقيقي.', shareEyebrow: 'نص جاهز للمشاركة', shareTitle: 'انسخ المخرجات والصقها مباشرة في الدردشة أو المستندات أو الملاحظات.', copyBtn: 'نسخ النص', copySuccess: 'تم نسخ النص إلى الحافظة.', downloadsEyebrow: 'دعم المنصات', downloadsTitle: 'نزّل النسخة المناسبة لسطح مكتبك.', previewEyebrow: 'معاينة', previewTitle: 'مصمم للملاحظات الجادة والملخصات وإعادة استخدام المحتوى.', ossEyebrow: 'مفتوح المصدر', ossTitle: 'تدفق عمل محلي أولًا. تجربة سطح مكتب أصلية. مكدس شفاف.', faqTitle: 'إجابات سريعة قبل التنزيل.', footerText: 'حوّل الفيديوهات إلى ملخصات ورؤى ونصوص قابلة للمشاركة.', footerNote: 'ClipTube AI. صُمّم للفهم السريع وإعادة الاستخدام.'
};
translations.hi = {
  ...translations.en,
  navFeatures: 'फ़ीचर्स', navWorkflow: 'यह कैसे काम करता है', navDownloads: 'डाउनलोड',
  heroTitle: 'लंबे YouTube वीडियो को दोबारा उपयोग होने वाली समझ में बदलें।',
  heroText: 'ClipTube AI ट्रांसक्रिप्ट निकालता है, सारांश बनाता है, मुख्य बिंदु दिखाता है, संदर्भ-आधारित AI चैट देता है और वीडियो ज्ञान को कॉपी, सुधार और शेयर करने लायक टेक्स्ट में बदलता है।',
  downloadBtn: 'डाउनलोड', webAppBtn: 'वेब ऐप खोलें', githubBtn: 'GitHub पर देखें', heroMeta1: 'क्रिएटर्स के लिए', heroMeta2: 'छात्रों के लिए', heroMeta3: 'शोधकर्ताओं के लिए', heroMeta4: 'प्रोडक्टिविटी प्रेमियों के लिए',
  heroBadge1: 'AI चैट', heroBadge2: 'शेयर के लिए तैयार',
  metric1Title: 'पूरा ट्रांसक्रिप्ट', metric1Text: 'वीडियो टेक्स्ट खींचें और स्रोत से काम करें।', metric2Title: 'Ollama-नेटिव', metric2Text: 'अपने इंस्टॉल किए गए लोकल कैटलॉग से सीधे मॉडल चुनें।', metric3Title: 'क्रॉस-प्लेटफ़ॉर्म', metric3Text: 'Linux / macOS / Windows के लिए डेस्कटॉप रिलीज़ फ्लो तैयार है।',
  featuresEyebrow: 'मुख्य क्षमताएँ', featuresTitle: '“इसे बाद में देखूँगा” से “मैंने पहले ही इसका मूल्य निकाल लिया” तक जाने के लिए जो कुछ चाहिए, सब कुछ।', feature1Title: 'पहले ट्रांसक्रिप्ट', feature1Text: 'YouTube URL या ID पेस्ट करें और पूरे ट्रांसक्रिप्ट को भरोसेमंद स्रोत के रूप में पाएँ।', feature2Title: 'AI सारांश', feature2Text: 'समझने, स्कैन करने और दोबारा उपयोग करने के लिए साफ-सुथरे सारांश बनाएँ।', feature3Title: 'मुख्य बिंदु', feature3Text: 'पूरा वीडियो दोबारा देखे बिना याद रखने योग्य विचार निकालें।', feature4Title: 'संदर्भित AI चैट', feature4Text: 'वीडियो के बारे में सीधे सवाल पूछें और आउटपुट को ऐप छोड़े बिना सुधारें।', feature5Title: 'पेस्ट-रेडी टेक्स्ट', feature5Text: 'चैट, नोट्स, डॉक्यूमेंट्स या पोस्ट के लिए साफ ब्लॉक्स तैयार करें।', feature6Title: 'Ollama मॉडल चयनकर्ता', feature6Text: 'Ollama से उपलब्ध मॉडल लोड करें और UI से बदलें।', feature7Title: 'बहुभाषी', feature7Text: 'सिस्टम भाषा पहचान और बहुभाषी समर्थन शुरुआत से अनुभव बेहतर बनाते हैं।', feature8Title: 'नेटिव Rust डेस्कटॉप', feature8Text: 'Rust + eframe/egui से बना तेज़, लोकल-फर्स्ट डेस्कटॉप वर्कफ़्लो।',
  workflowEyebrow: 'यह कैसे काम करता है', workflowTitle: 'दैनिक उपयोग के लिए काफी तेज़। असली काम के लिए पर्याप्त संरचित।', shareEyebrow: 'शेयर-रेडी टेक्स्ट', shareTitle: 'आउटपुट कॉपी करें और उसे सीधे चैट, डॉक्यूमेंट या नोट्स में डालें।', copyBtn: 'टेक्स्ट कॉपी करें', copySuccess: 'टेक्स्ट क्लिपबोर्ड में कॉपी हो गया।', downloadsEyebrow: 'प्लेटफ़ॉर्म सपोर्ट', downloadsTitle: 'अपने डेस्कटॉप के लिए सही बिल्ड डाउनलोड करें।', previewEyebrow: 'प्रीव्यू', previewTitle: 'गंभीर नोट-टेकिंग, सारांश और कंटेंट री-यूज़ के लिए बनाया गया।', ossEyebrow: 'ओपन सोर्स', ossTitle: 'लोकल-फर्स्ट वर्कफ़्लो। नेटिव डेस्कटॉप अनुभव। पारदर्शी स्टैक।', faqTitle: 'डाउनलोड से पहले तेज़ जवाब।', footerText: 'वीडियो को ट्रांसक्रिप्ट-आधारित सारांश, इनसाइट्स और शेयर करने योग्य आउटपुट में बदलें।', footerNote: 'ClipTube AI. तेज़ समझ और पुनः उपयोग के लिए बनाया गया।'
};

Object.assign(translations.en, {
  navWebApp: 'Web App',
  webLabel: '🌐 Web',
  webTitle: 'Browser app',
  webText: 'Open the web runtime with Ollama connection, transcript backend support and the same core flow as the desktop app.',
  webBtn: 'Open web app',
  setupEyebrow: 'Setup',
  setupTitle: 'Configure ClipTube AI in a few minutes.',
  setupText: 'Whether you use the desktop app or the web runtime, the basic setup is the same: make sure Ollama is running, point ClipTube AI to the right host and port, and use the transcript backend when you want reliable browser-side transcript fetching.',
  setup1Title: 'Start Ollama',
  setup1Text: 'Run your local Ollama instance and confirm your models are available. The default ClipTube AI connection expects 127.0.0.1:11434.',
  setup2Title: 'Configure host and port',
  setup2Text: 'In the app settings, set the Ollama host and port, or provide a full endpoint override if you use a remote instance.',
  setup3Title: 'Use the transcript backend for the web app',
  setup3Text: 'For the browser version, run the local runtime server and keep the transcript backend pointing to the same origin so YouTube transcript requests can be resolved server-side.',
  setup4Title: 'Pick your model and output style',
  setup4Text: 'Once the connection is healthy, choose a model, fetch the transcript, and generate summaries, key points or share-ready text.',
  setupCommandLabel: 'Recommended local command for the web runtime:',
  setupCommandTitle: 'Run the local web runtime',
  setupCommand: 'node scripts/web_runtime_server.js',
  setupOfflineTitle: 'Installable PWA + local full mode',
  setupOfflineText: 'On GitHub Pages, ClipTube AI Web works as an installable PWA and can cache the interface offline. For full transcript fetching and Ollama-powered analysis, use the local web runtime.',
});

Object.assign(translations.es, {
  navWebApp: 'App web',
  webLabel: '🌐 Web',
  webTitle: 'App web',
  webText: 'Abre la versión web con conexión a Ollama, soporte de backend para transcript y el mismo flujo central que la app de escritorio.',
  webBtn: 'Abrir app web',
  setupEyebrow: 'Configuración',
  setupTitle: 'Configura ClipTube AI en pocos minutos.',
  setupText: 'Da igual si usas la app de escritorio o la versión web: la base es la misma. Asegúrate de que Ollama esté corriendo, apunta ClipTube AI al host y puerto correctos, y usa el backend de transcript cuando quieras obtener transcripciones fiables desde el navegador.',
  setup1Title: 'Inicia Ollama',
  setup1Text: 'Ejecuta tu instancia local de Ollama y confirma que tus modelos estén disponibles. La conexión por defecto de ClipTube AI espera 127.0.0.1:11434.',
  setup2Title: 'Configura host y puerto',
  setup2Text: 'En Ajustes, define el host y el puerto de Ollama, o usa un endpoint completo si trabajas con una instancia remota.',
  setup3Title: 'Usa el backend de transcript en la web',
  setup3Text: 'Para la versión web, ejecuta el runtime local y deja el backend de transcript apuntando al mismo origen para que las peticiones a YouTube se resuelvan del lado del servidor.',
  setup4Title: 'Elige modelo y estilo de salida',
  setup4Text: 'Cuando la conexión esté sana, elige un modelo, obtén el transcript y genera resumen, puntos importantes o texto listo para compartir.',
  setupCommandLabel: 'Comando local recomendado para la app web:',
  setupCommandTitle: 'Levantar el runtime web local',
  setupCommand: 'node scripts/web_runtime_server.js',
  setupOfflineTitle: 'PWA instalable + modo local completo',
  setupOfflineText: 'En GitHub Pages, ClipTube AI Web funciona como PWA instalable y puede cachear la interfaz offline. Para transcripciones confiables y análisis con Ollama, usa el runtime web local.',
});

Object.assign(translations.pt, {
  navWebApp: 'App web',
  webLabel: '🌐 Web',
  webTitle: 'App web',
  webText: 'Abra a versão web com conexão ao Ollama, suporte a backend de transcrição e o mesmo fluxo principal da app desktop.',
  webBtn: 'Abrir app web',
  setupEyebrow: 'Configuração',
  setupTitle: 'Configure o ClipTube AI em poucos minutos.',
  setupText: 'Tanto na app desktop quanto na versão web, a base é a mesma: garanta que o Ollama esteja rodando, aponte o ClipTube AI para o host e porta corretos e use o backend de transcrição quando quiser mais confiabilidade no navegador.',
  setup1Title: 'Inicie o Ollama',
  setup1Text: 'Execute sua instância local do Ollama e confirme que os modelos estão disponíveis. A conexão padrão do ClipTube AI espera 127.0.0.1:11434.',
  setup2Title: 'Configure host e porta',
  setup2Text: 'Nas configurações, defina host e porta do Ollama, ou informe um endpoint completo se você usa uma instância remota.',
  setup3Title: 'Use o backend de transcrição na web',
  setup3Text: 'Na versão web, execute o runtime local e mantenha o backend de transcrição apontando para a mesma origem para que as requisições do YouTube sejam resolvidas no servidor.',
  setup4Title: 'Escolha modelo e estilo de saída',
  setup4Text: 'Quando a conexão estiver saudável, escolha um modelo, busque a transcrição e gere resumos, pontos importantes ou texto pronto para compartilhar.',
  setupCommandLabel: 'Comando local recomendado para a app web:',
  setupCommandTitle: 'Subir o runtime web local',
  setupCommand: 'node scripts/web_runtime_server.js',
  setupOfflineTitle: 'PWA instalável + modo local completo',
  setupOfflineText: 'No GitHub Pages, o ClipTube AI Web funciona como PWA instalável e pode armazenar a interface offline. Para transcrição confiável e análise com Ollama, use o runtime web local.',
});

Object.assign(translations.fr, {
  navWebApp: 'App web',
  webLabel: '🌐 Web',
  webTitle: 'App navigateur',
  webText: 'Ouvrez la version web avec connexion à Ollama, support du backend de transcript et le même flux principal que l’app desktop.',
  webBtn: 'Ouvrir l’app web',
  setupEyebrow: 'Configuration',
  setupTitle: 'Configurez ClipTube AI en quelques minutes.',
  setupText: 'Que vous utilisiez l’app desktop ou la version web, la base est la même : assurez-vous que Ollama tourne, pointez ClipTube AI vers le bon hôte et port, et utilisez le backend de transcript pour une récupération fiable dans le navigateur.',
  setup1Title: 'Démarrer Ollama',
  setup1Text: 'Lancez votre instance locale Ollama et vérifiez que vos modèles sont disponibles. La connexion par défaut de ClipTube AI attend 127.0.0.1:11434.',
  setup2Title: 'Configurer l’hôte et le port',
  setup2Text: 'Dans les réglages, définissez l’hôte et le port d’Ollama, ou fournissez un endpoint complet si vous utilisez une instance distante.',
  setup3Title: 'Utiliser le backend de transcript pour la version web',
  setup3Text: 'Pour la version navigateur, lancez le runtime local et gardez le backend de transcript sur la même origine afin que les requêtes YouTube soient résolues côté serveur.',
  setup4Title: 'Choisir le modèle et le style de sortie',
  setup4Text: 'Une fois la connexion saine, choisissez un modèle, récupérez la transcription et générez résumé, points clés ou texte prêt à partager.',
  setupCommandLabel: 'Commande locale recommandée pour l’app web :',
  setupCommandTitle: 'Lancer le runtime web local',
  setupCommand: 'node scripts/web_runtime_server.js',
  setupOfflineTitle: 'PWA installable + mode local complet',
  setupOfflineText: 'Sur GitHub Pages, ClipTube AI Web fonctionne comme une PWA installable et peut mettre l’interface en cache hors ligne. Pour des transcriptions fiables et l’analyse via Ollama, utilisez le runtime web local.',
});

Object.assign(translations.de, {
  navWebApp: 'Web-App',
  webLabel: '🌐 Web',
  webTitle: 'Browser-App',
  webText: 'Öffne die Web-Version mit Ollama-Verbindung, Transcript-Backend-Unterstützung und demselben Kern-Workflow wie in der Desktop-App.',
  webBtn: 'Web-App öffnen',
  setupEyebrow: 'Einrichtung',
  setupTitle: 'Richte ClipTube AI in wenigen Minuten ein.',
  setupText: 'Egal ob Desktop-App oder Web-Version: die Grundlage ist gleich. Stelle sicher, dass Ollama läuft, richte Host und Port korrekt ein und nutze das Transcript-Backend für zuverlässige Browser-Transkripte.',
  setup1Title: 'Ollama starten',
  setup1Text: 'Starte deine lokale Ollama-Instanz und prüfe, ob deine Modelle verfügbar sind. Die Standardverbindung von ClipTube AI erwartet 127.0.0.1:11434.',
  setup2Title: 'Host und Port konfigurieren',
  setup2Text: 'Lege in den Einstellungen Host und Port von Ollama fest oder gib einen vollständigen Endpoint an, wenn du eine entfernte Instanz nutzt.',
  setup3Title: 'Transcript-Backend für die Web-App verwenden',
  setup3Text: 'Für die Browser-Version starte den lokalen Runtime-Server und lasse das Transcript-Backend auf dieselbe Origin zeigen, damit YouTube-Anfragen serverseitig aufgelöst werden.',
  setup4Title: 'Modell und Ausgabestil wählen',
  setup4Text: 'Sobald die Verbindung steht, wähle ein Modell, hole das Transkript und erzeuge Zusammenfassungen, Kernpunkte oder teilbaren Text.',
  setupCommandLabel: 'Empfohlener lokaler Befehl für die Web-App:',
  setupCommandTitle: 'Lokale Web-Runtime starten',
  setupCommand: 'node scripts/web_runtime_server.js',
  setupOfflineTitle: 'Installierbare PWA + voller lokaler Modus',
  setupOfflineText: 'Auf GitHub Pages funktioniert ClipTube AI Web als installierbare PWA und kann die Oberfläche offline cachen. Für zuverlässige Transkripte und Ollama-Analyse nutze den lokalen Web-Runtime.',
});

Object.assign(translations.ja, {
  navWebApp: 'Webアプリ',
  webLabel: '🌐 Web',
  webTitle: 'ブラウザ版アプリ',
  webText: 'Ollama 接続、文字起こしバックエンド対応、そしてデスクトップ版と同じ基本フローを備えた Web 版を開きます。',
  webBtn: 'Webアプリを開く',
  setupEyebrow: 'セットアップ',
  setupTitle: 'ClipTube AI を数分で設定できます。',
  setupText: 'デスクトップ版でも Web 版でも基本は同じです。Ollama を起動し、正しいホストとポートを設定し、ブラウザで安定して文字起こしを取得したい場合は transcript backend を使ってください。',
  setup1Title: 'Ollama を起動する',
  setup1Text: 'ローカルの Ollama インスタンスを起動し、モデルが利用可能であることを確認してください。ClipTube AI の既定接続は 127.0.0.1:11434 を想定しています。',
  setup2Title: 'ホストとポートを設定する',
  setup2Text: '設定画面で Ollama のホストとポートを指定するか、リモート環境を使う場合は完全なエンドポイントを指定してください。',
  setup3Title: 'Web 版では transcript backend を使う',
  setup3Text: 'ブラウザ版ではローカルの runtime server を起動し、transcript backend を同じオリジンに向けることで YouTube 文字起こし取得をサーバー側で処理できます。',
  setup4Title: 'モデルと出力スタイルを選ぶ',
  setup4Text: '接続が正常になったら、モデルを選び、文字起こしを取得し、要約・重要ポイント・共有用テキストを生成します。',
  setupCommandLabel: 'Web 版に推奨されるローカルコマンド:',
  setupCommandTitle: 'ローカル Web ランタイムを起動',
  setupCommand: 'node scripts/web_runtime_server.js',
  setupOfflineTitle: 'インストール可能な PWA + ローカル完全モード',
  setupOfflineText: 'GitHub Pages では ClipTube AI Web はインストール可能な PWA として動作し、UI をオフラインでキャッシュできます。信頼できる文字起こし取得と Ollama 分析にはローカル Web ランタイムを使ってください。',
});

Object.assign(translations['zh-Hans'], {
  navWebApp: 'Web 应用',
  webLabel: '🌐 Web',
  webTitle: '浏览器应用',
  webText: '打开带有 Ollama 连接、转录后端支持，并与桌面版共享核心流程的 Web 版本。',
  webBtn: '打开 Web 应用',
  setupEyebrow: '配置',
  setupTitle: '几分钟内完成 ClipTube AI 配置。',
  setupText: '无论你使用桌面版还是 Web 版，基础配置都是一样的：确保 Ollama 正在运行，将 ClipTube AI 指向正确的主机和端口，并在浏览器中需要稳定转录时使用 transcript backend。',
  setup1Title: '启动 Ollama',
  setup1Text: '启动本地 Ollama 实例，并确认模型可用。ClipTube AI 默认连接使用 127.0.0.1:11434。',
  setup2Title: '配置主机和端口',
  setup2Text: '在设置中填写 Ollama 的主机和端口；如果你使用远程实例，也可以直接填写完整 endpoint。',
  setup3Title: 'Web 版使用 transcript backend',
  setup3Text: '对于浏览器版本，请运行本地 runtime server，并让 transcript backend 指向相同来源，这样 YouTube 转录请求就能在服务器端处理。',
  setup4Title: '选择模型和输出样式',
  setup4Text: '连接正常后，选择模型、获取转录，然后生成摘要、关键点或可分享文本。',
  setupCommandLabel: 'Web 版推荐的本地命令：',
  setupCommandTitle: '启动本地 Web runtime',
  setupCommand: 'node scripts/web_runtime_server.js',
  setupOfflineTitle: '可安装 PWA + 本地完整模式',
  setupOfflineText: '在 GitHub Pages 上，ClipTube AI Web 可作为可安装 PWA 运行，并离线缓存界面。若要获得可靠转录和基于 Ollama 的完整分析，请使用本地 Web runtime。',
});

Object.assign(translations.ru, {
  navWebApp: 'Веб-приложение',
  webLabel: '🌐 Web',
  webTitle: 'Браузерное приложение',
  webText: 'Откройте веб-версию с подключением к Ollama, поддержкой backend для транскриптов и тем же основным сценарием работы, что и в desktop-приложении.',
  webBtn: 'Открыть веб-приложение',
  setupEyebrow: 'Настройка',
  setupTitle: 'Настройте ClipTube AI за несколько минут.',
  setupText: 'Неважно, используете ли вы desktop-приложение или веб-версию: базовая настройка одинаковая. Убедитесь, что Ollama запущен, укажите правильный host и port, а для надёжной загрузки транскриптов в браузере используйте transcript backend.',
  setup1Title: 'Запустите Ollama',
  setup1Text: 'Запустите локальный экземпляр Ollama и убедитесь, что модели доступны. По умолчанию ClipTube AI ожидает соединение с 127.0.0.1:11434.',
  setup2Title: 'Настройте host и port',
  setup2Text: 'В настройках укажите host и port Ollama, либо задайте полный endpoint, если используете удалённый экземпляр.',
  setup3Title: 'Используйте transcript backend для веб-версии',
  setup3Text: 'Для браузерной версии запустите локальный runtime server и оставьте transcript backend на том же origin, чтобы запросы к YouTube обрабатывались на стороне сервера.',
  setup4Title: 'Выберите модель и стиль вывода',
  setup4Text: 'Когда соединение работает, выберите модель, получите транскрипт и создайте сводку, ключевые пункты или готовый к отправке текст.',
  setupCommandLabel: 'Рекомендуемая локальная команда для веб-версии:',
  setupCommandTitle: 'Запустить локальный web runtime',
  setupCommand: 'node scripts/web_runtime_server.js',
  setupOfflineTitle: 'Устанавливаемая PWA + полный локальный режим',
  setupOfflineText: 'На GitHub Pages ClipTube AI Web работает как устанавливаемая PWA и может кэшировать интерфейс офлайн. Для надёжных транскриптов и анализа через Ollama используйте локальный web runtime.',
});

Object.assign(translations.ar, {
  navWebApp: 'تطبيق الويب',
  webLabel: '🌐 Web',
  webTitle: 'تطبيق المتصفح',
  webText: 'افتح النسخة web مع اتصال Ollama ودعم backend للنص المفرغ وبنفس التدفق الأساسي الموجود في تطبيق desktop.',
  webBtn: 'افتح تطبيق الويب',
  setupEyebrow: 'الإعداد',
  setupTitle: 'اضبط ClipTube AI خلال دقائق قليلة.',
  setupText: 'سواء كنت تستخدم تطبيق desktop أو النسخة web، فالإعداد الأساسي واحد: تأكد من تشغيل Ollama، وجّه ClipTube AI إلى المضيف والمنفذ الصحيحين، واستخدم transcript backend عندما تريد جلب نصوص موثوقة من داخل المتصفح.',
  setup1Title: 'شغّل Ollama',
  setup1Text: 'شغّل نسخة Ollama المحلية وتأكد من أن النماذج متاحة. الاتصال الافتراضي في ClipTube AI يتوقع 127.0.0.1:11434.',
  setup2Title: 'اضبط المضيف والمنفذ',
  setup2Text: 'من الإعدادات حدّد host و port الخاصين بـ Ollama، أو استخدم endpoint كامل إذا كنت تعمل على نسخة بعيدة.',
  setup3Title: 'استخدم transcript backend في نسخة الويب',
  setup3Text: 'في نسخة المتصفح شغّل runtime server المحلي واجعل transcript backend يشير إلى نفس الأصل حتى تتم معالجة طلبات YouTube على جهة الخادم.',
  setup4Title: 'اختر النموذج ونمط الإخراج',
  setup4Text: 'عندما تصبح حالة الاتصال سليمة، اختر نموذجًا، اجلب النص المفرغ، ثم أنشئ ملخصات أو نقاطًا مهمة أو نصًا جاهزًا للمشاركة.',
  setupCommandLabel: 'الأمر المحلي الموصى به لنسخة الويب:',
  setupCommandTitle: 'تشغيل Web Runtime المحلي',
  setupCommand: 'node scripts/web_runtime_server.js',
  setupOfflineTitle: 'تطبيق PWA قابل للتثبيت + وضع محلي كامل',
  setupOfflineText: 'على GitHub Pages يعمل ClipTube AI Web كتطبيق PWA قابل للتثبيت ويمكنه حفظ الواجهة للعمل دون اتصال. للحصول على جلب موثوق للنصوص وتحليل كامل عبر Ollama، استخدم بيئة الويب المحلية.',
});

Object.assign(translations.hi, {
  navWebApp: 'वेब ऐप',
  webLabel: '🌐 Web',
  webTitle: 'ब्राउज़र ऐप',
  webText: 'Ollama कनेक्शन, transcript backend सपोर्ट और desktop ऐप जैसे ही मुख्य फ्लो वाली web version खोलें।',
  webBtn: 'वेब ऐप खोलें',
  setupEyebrow: 'सेटअप',
  setupTitle: 'कुछ ही मिनटों में ClipTube AI कॉन्फ़िगर करें।',
  setupText: 'चाहे आप desktop app उपयोग करें या web version, मूल सेटअप समान है: सुनिश्चित करें कि Ollama चल रहा है, ClipTube AI को सही host और port दें, और browser में भरोसेमंद transcript fetch के लिए transcript backend का उपयोग करें।',
  setup1Title: 'Ollama शुरू करें',
  setup1Text: 'अपना local Ollama instance चलाएँ और पुष्टि करें कि मॉडल उपलब्ध हैं। ClipTube AI का डिफ़ॉल्ट कनेक्शन 127.0.0.1:11434 अपेक्षित करता है।',
  setup2Title: 'Host और port कॉन्फ़िगर करें',
  setup2Text: 'सेटिंग्स में Ollama का host और port सेट करें, या यदि आप remote instance उपयोग करते हैं तो पूरा endpoint दें।',
  setup3Title: 'Web app के लिए transcript backend उपयोग करें',
  setup3Text: 'ब्राउज़र version में local runtime server चलाएँ और transcript backend को उसी origin पर रखें ताकि YouTube transcript requests सर्वर-साइड हल हो सकें।',
  setup4Title: 'Model और output style चुनें',
  setup4Text: 'कनेक्शन ठीक होने के बाद मॉडल चुनें, transcript प्राप्त करें और summary, key points या share-ready text बनाएँ।',
  setupCommandLabel: 'Web app के लिए सुझाया गया local command:',
  setupCommandTitle: 'Local web runtime चलाएँ',
  setupCommand: 'node scripts/web_runtime_server.js',
  setupOfflineTitle: 'इंस्टॉल होने वाली PWA + पूरा लोकल मोड',
  setupOfflineText: 'GitHub Pages पर ClipTube AI Web एक इंस्टॉल होने वाली PWA की तरह काम करता है और इंटरफ़ेस को ऑफलाइन कैश कर सकता है। भरोसेमंद ट्रांसक्रिप्ट और Ollama विश्लेषण के लिए लोकल वेब runtime का उपयोग करें।',
});

const previewImageMap = {
  en: 'assets/screenshots/app-preview-en.png',
  es: 'assets/screenshots/app-preview-es.png',
  pt: 'assets/screenshots/app-preview-pt.png',
  fr: 'assets/screenshots/app-preview-fr.png',
  de: 'assets/screenshots/app-preview-de.png',
  ja: 'assets/screenshots/app-preview-ja.png',
  'zh-Hans': 'assets/screenshots/app-preview-zh-Hans.png',
  ru: 'assets/screenshots/app-preview-ru.png',
  ar: 'assets/screenshots/app-preview-ar.png',
  hi: 'assets/screenshots/app-preview-hi.png'
};

function normalizeLanguageCode(raw) {
  const value = (raw || 'en').toLowerCase();
  if (value.startsWith('zh')) return 'zh-Hans';
  if (value.startsWith('pt')) return 'pt';
  if (value.startsWith('es')) return 'es';
  if (value.startsWith('fr')) return 'fr';
  if (value.startsWith('de')) return 'de';
  if (value.startsWith('ja')) return 'ja';
  if (value.startsWith('ru')) return 'ru';
  if (value.startsWith('ar')) return 'ar';
  if (value.startsWith('hi')) return 'hi';
  if (value.startsWith('en')) return 'en';
  return 'en';
}

function getDictionary(lang) {
  return translations[lang] || translations.en;
}

function setPreviewImages(lang) {
  const primary = previewImageMap[lang] || previewImageMap.en;
  const fallback = previewImageMap.en;
  const finalFallback = 'assets/app-preview.svg';

  document.querySelectorAll('[data-preview-image]').forEach((img) => {
    img.onerror = () => {
      if (!img.dataset.fallbackStep || img.dataset.fallbackStep === '0') {
        img.dataset.fallbackStep = '1';
        img.src = fallback;
        return;
      }
      img.dataset.fallbackStep = '2';
      img.onerror = null;
      img.src = finalFallback;
    };
    img.dataset.fallbackStep = '0';
    img.src = primary;
  });
}

function detectLanguage() {
  const saved = localStorage.getItem('cliptube-language');
  if (saved && translations[saved]) return saved;
  return normalizeLanguageCode(navigator.language || 'en');
}

function applyLanguage(lang) {
  const dict = getDictionary(lang);
  document.documentElement.lang = lang;

  document.querySelectorAll('[data-i18n]').forEach((node) => {
    const key = node.dataset.i18n;
    if (dict[key]) node.textContent = dict[key];
  });

  document.querySelectorAll('[data-i18n-value]').forEach((node) => {
    const key = node.dataset.i18nValue;
    if (dict[key]) node.value = dict[key];
  });

  const select = document.getElementById('language-select');
  if (select) select.value = lang;

  localStorage.setItem('cliptube-language', lang);
  setPreviewImages(lang);
}

for (const link of document.querySelectorAll('[data-repo-link]')) {
  link.href = repoUrl;
}
for (const link of document.querySelectorAll('[data-release-link]')) {
  link.href = releasesUrl;
}

document.getElementById('year').textContent = new Date().getFullYear();

const feedback = document.querySelector('[data-copy-feedback]');
for (const button of document.querySelectorAll('[data-copy-target]')) {
  button.addEventListener('click', async () => {
    const target = document.getElementById(button.dataset.copyTarget);
    if (!target) return;
    const currentLang = localStorage.getItem('cliptube-language') || detectLanguage();
    const dict = getDictionary(currentLang);

    try {
      await navigator.clipboard.writeText(target.value);
      if (feedback) feedback.textContent = dict.copySuccess;
    } catch (error) {
      target.select();
      document.execCommand('copy');
      if (feedback) feedback.textContent = dict.copySuccess;
    }
  });
}

document.getElementById('language-select')?.addEventListener('change', (event) => {
  applyLanguage(event.target.value);
});

applyLanguage(detectLanguage());

function getTopbarOffset() {
  const topbar = document.querySelector('.topbar');
  if (!topbar) return 92;
  return Math.ceil(topbar.getBoundingClientRect().height + 20);
}

function getSectionAnchorTarget(section) {
  return section?.querySelector('.section-head') || section;
}

function scrollToSection(section) {
  const target = getSectionAnchorTarget(section);
  if (!target) return;
  const y = target.getBoundingClientRect().top + window.scrollY - getTopbarOffset();
  window.scrollTo({ top: Math.max(0, y), behavior: 'smooth' });
}

function setActiveNavLink(sectionId) {
  document.querySelectorAll('.nav a[href^="#"]').forEach((link) => {
    link.classList.toggle('active', link.getAttribute('href') === `#${sectionId}`);
  });
}

function setupSectionNavigation() {
  const navLinks = [...document.querySelectorAll('.nav a[href^="#"]')];
  const sections = navLinks
    .map((link) => document.querySelector(link.getAttribute('href')))
    .filter(Boolean);

  navLinks.forEach((link) => {
    link.addEventListener('click', (event) => {
      const section = document.querySelector(link.getAttribute('href'));
      if (!section) return;
      event.preventDefault();
      scrollToSection(section);
      setActiveNavLink(section.id);
      history.replaceState(null, '', `#${section.id}`);
    });
  });

  if (!sections.length) return;

  const onScroll = () => {
    const offset = getTopbarOffset() + 24;
    let current = sections[0];
    for (const section of sections) {
      const target = getSectionAnchorTarget(section);
      if (!target) continue;
      if (target.getBoundingClientRect().top - offset <= 0) current = section;
    }
    setActiveNavLink(current.id);
  };

  window.addEventListener('scroll', onScroll, { passive: true });
  window.addEventListener('resize', onScroll);
  onScroll();

  if (window.location.hash) {
    const section = document.querySelector(window.location.hash);
    if (section) setTimeout(() => scrollToSection(section), 0);
  }
}

setupSectionNavigation();
