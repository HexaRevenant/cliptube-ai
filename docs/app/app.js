/* ══════════════════════════════════════════════════════
   ClipTube AI — Web App
   Browser-side runtime aligned with the desktop flow.
   ══════════════════════════════════════════════════════ */

const LANGUAGE_OPTIONS = [
  ['es', '🇪🇸 Español'],
  ['en', '🇺🇸 English'],
  ['pt', '🇧🇷 Português'],
  ['fr', '🇫🇷 Français'],
  ['de', '🇩🇪 Deutsch'],
  ['ja', '🇯🇵 日本語'],
  ['zh-Hans', '🇨🇳 简体中文'],
  ['ru', '🇷🇺 Русский'],
  ['ar', '🇸🇦 العربية'],
  ['hi', '🇮🇳 हिन्दी'],
];

const TRANSCRIPT_LANGUAGE_DEFAULTS = {
  es: 'es,en',
  en: 'en,es',
  pt: 'pt,es,en',
  fr: 'fr,en,es',
  de: 'de,en,es',
  ja: 'ja,en',
  'zh-Hans': 'zh,en',
  ru: 'ru,en',
  ar: 'ar,en',
  hi: 'hi,en',
};

const i18n = {
  es: {
    settingsTitle: 'Conexión Ollama',
    settingsHint: 'Configura host, puerto o un endpoint completo de Ollama. Si defines un override, tendrá prioridad.',
    settingsUrl: 'Endpoint completo (override)',
    settingsProxy: 'Proxy CORS (transcripts)',
    settingsTranscriptBackend: 'Backend transcript (opcional)',
    settingsHostPort: 'Host y puerto',
    settingsEffectiveUrl: 'URL efectiva',
    settingsTest: 'Probar conexión',
    settingsSave: 'Guardar',
    urlLabel: 'URL O ID DE YOUTUBE',
    languagesLabel: 'IDIOMAS DEL TRANSCRIPT',
    modelLabel: 'MODELO',
    styleLabel: 'ESTILO DE SALIDA',
    styleChat: 'Chat listo para pegar',
    styleExec: 'Resumen ejecutivo',
    styleBullets: 'Bullets directos',
    btnFetch: 'Obtener transcript',
    btnAnalyze: 'Generar resumen',
    btnImprove: 'Mejorar texto final',
    metaTitle: 'Metadatos',
    summaryTitle: 'Resumen',
    keyPointsTitle: 'Puntos importantes',
    transcriptTitle: 'Transcripción',
    shareTitle: 'Texto listo para pegar',
    btnShow: 'Mostrar',
    btnHide: 'Ocultar',
    btnCopy: 'Copiar',
    copied: '✓ Copiado al portapapeles',
    chatTitle: 'Chat IA',
    chatPlaceholder: 'Pregunta algo sobre el video. El chat usa el transcript y el texto final actual.',
    chatInput: 'Ej: mejora el gancho inicial, hazlo más ejecutivo, responde una duda del video…',
    btnSend: 'Preguntar a la IA',
    loading: 'Procesando…',
    loadingTranscript: 'Obteniendo transcripción…',
    loadingAnalysis: 'Analizando con IA…',
    loadingChat: 'Consultando la IA…',
    loadingImprove: 'Mejorando texto final…',
    errOllama: 'No se pudo conectar a Ollama.',
    errTranscript: 'No se pudo obtener la transcripción.',
    errModel: 'Error cargando modelos.',
    errChat: 'Error en el chat. Intenta de nuevo.',
    errInvalidConnection: 'Configura un host y puerto válidos, o un endpoint completo correcto.',
    connSuccess: '✓ Conectado a Ollama',
    connFail: '✗ No se pudo conectar',
    noTranscript: 'Primero obtén la transcripción del video.',
    noModel: 'Selecciona un modelo.',
    noQuestion: 'Escribe una pregunta para la IA.',
    fetchSuccess: '✓ Transcripción obtenida',
    improveSuccess: '✓ Texto final mejorado',
    chatReady: '✓ Respuesta lista',
    modelCount: (n) => `${n} modelo${n !== 1 ? 's' : ''} disponible${n !== 1 ? 's' : ''}`,
    metaTranscriptChars: 'Transcript',
    metaOutput: 'Salida',
    assistantLabel: 'IA',
    userLabel: 'Tú',
    btnInstall: 'Instalar app',
    btnInstalled: 'App instalada',
    installUnavailable: 'A instalação não está disponível neste contexto.',
    installAccepted: '✓ Instalação aceita',
    installDismissed: 'Instalação cancelada',
    installManualHint: 'Se o diálogo não aparecer, use o menu do navegador para instalar o app.',
    runtimeBannerTitle: 'App web instalable',
    runtimeBannerRemote: 'Instala la PWA para abrir la interfaz más rápido y usarla offline. Para transcripciones confiables y conexión total con Ollama, ejecuta el runtime web local o apunta el backend transcript a ese runtime.',
    runtimeBannerLocal: 'Estás usando el runtime local completo. La interfaz puede instalarse como PWA y seguir abriendo rápido, mientras el transcript y Ollama se resuelven en tu entorno local.',
    runtimePillRemote: 'Modo interfaz / Pages',
    runtimePillLocal: 'Modo local completo',
    runtimeInstalledIcon: '✅',
    runtimeBannerInstalledLocal: 'Estás usando la app ya instalada con el runtime local completo. Transcript y Ollama se resuelven en tu entorno local.',
    runtimeBannerInstalledRemote: 'Estás usando la app ya instalada. La interfaz puede abrir offline, pero para transcript confiable y Ollama completo conviene usar el runtime web local.',
  },
  en: {
    settingsTitle: 'Ollama Connection',
    settingsHint: 'Configure Ollama via host/port or a full endpoint override. If override is set, it takes precedence.',
    settingsUrl: 'Full endpoint (override)',
    settingsProxy: 'CORS Proxy (transcripts)',
    settingsTranscriptBackend: 'Transcript backend (optional)',
    settingsHostPort: 'Host and port',
    settingsEffectiveUrl: 'Effective URL',
    settingsTest: 'Test connection',
    settingsSave: 'Save',
    urlLabel: 'YOUTUBE URL OR ID',
    languagesLabel: 'TRANSCRIPT LANGUAGES',
    modelLabel: 'MODEL',
    styleLabel: 'OUTPUT STYLE',
    styleChat: 'Ready-to-paste chat',
    styleExec: 'Executive summary',
    styleBullets: 'Direct bullets',
    btnFetch: 'Fetch transcript',
    btnAnalyze: 'Generate summary',
    btnImprove: 'Improve final text',
    metaTitle: 'Metadata',
    summaryTitle: 'Summary',
    keyPointsTitle: 'Key points',
    transcriptTitle: 'Transcript',
    shareTitle: 'Ready-to-paste text',
    btnShow: 'Show',
    btnHide: 'Hide',
    btnCopy: 'Copy',
    copied: '✓ Copied to clipboard',
    chatTitle: 'AI Chat',
    chatPlaceholder: 'Ask about the video. The chat uses the transcript and current final text.',
    chatInput: 'Ex: improve the opening hook, make it more executive, answer a question about the video…',
    btnSend: 'Ask AI',
    loading: 'Processing…',
    loadingTranscript: 'Fetching transcript…',
    loadingAnalysis: 'Analyzing with AI…',
    loadingChat: 'Consulting AI…',
    loadingImprove: 'Improving final text…',
    errOllama: 'Could not connect to Ollama.',
    errTranscript: 'Could not fetch the transcript.',
    errModel: 'Error loading models.',
    errChat: 'Chat error. Please try again.',
    errInvalidConnection: 'Set a valid host and port, or a valid full endpoint override.',
    connSuccess: '✓ Connected to Ollama',
    connFail: '✗ Could not connect',
    noTranscript: 'Fetch the video transcript first.',
    noModel: 'Select a model.',
    noQuestion: 'Type a question for the AI.',
    fetchSuccess: '✓ Transcript fetched',
    improveSuccess: '✓ Final text improved',
    chatReady: '✓ Reply ready',
    modelCount: (n) => `${n} model${n !== 1 ? 's' : ''} available`,
    metaTranscriptChars: 'Transcript',
    metaOutput: 'Output',
    assistantLabel: 'AI',
    userLabel: 'You',
    btnInstall: 'Install app',
    btnInstalled: 'App installed',
    installUnavailable: 'Install is not available in this context.',
    installAccepted: '✓ Install accepted',
    installDismissed: 'Install canceled',
    installManualHint: 'If no prompt appears, use the browser menu to install the app.',
    runtimeBannerTitle: 'Installable web app',
    runtimeBannerRemote: 'Install the PWA for fast access and offline UI. For reliable transcript fetching and full Ollama integration, run the local web runtime or point the transcript backend to it.',
    runtimeBannerLocal: 'You are using the full local runtime. Install the PWA for quick launching while transcript fetching and Ollama stay resolved in your local environment.',
    runtimePillRemote: 'UI / Pages mode',
    runtimePillLocal: 'Full local mode',
    runtimeInstalledIcon: '✅',
    runtimeBannerInstalledLocal: 'You are using the installed app with the full local runtime. Transcript fetching and Ollama stay resolved in your local environment.',
    runtimeBannerInstalledRemote: 'You are using the installed app. The interface can open offline, but reliable transcript fetching and full Ollama integration work best with the local web runtime.',
  },
  pt: {
    settingsTitle: 'Conexão Ollama',
    settingsHint: 'Configure o Ollama por host/porta ou por endpoint completo. Se houver override, ele terá prioridade.',
    settingsUrl: 'Endpoint completo (override)',
    settingsProxy: 'Proxy CORS (transcrições)',
    settingsTranscriptBackend: 'Backend de transcrição (opcional)',
    settingsHostPort: 'Host e porta',
    settingsEffectiveUrl: 'URL efetiva',
    settingsTest: 'Testar conexão',
    settingsSave: 'Salvar',
    urlLabel: 'URL OU ID DO YOUTUBE',
    languagesLabel: 'IDIOMAS DA TRANSCRIÇÃO',
    modelLabel: 'MODELO',
    styleLabel: 'ESTILO DE SAÍDA',
    styleChat: 'Chat pronto para colar',
    styleExec: 'Resumo executivo',
    styleBullets: 'Bullets diretos',
    btnFetch: 'Buscar transcrição',
    btnAnalyze: 'Gerar resumo',
    btnImprove: 'Melhorar texto final',
    metaTitle: 'Metadados',
    summaryTitle: 'Resumo',
    keyPointsTitle: 'Pontos importantes',
    transcriptTitle: 'Transcrição',
    shareTitle: 'Texto pronto para colar',
    btnShow: 'Mostrar',
    btnHide: 'Ocultar',
    btnCopy: 'Copiar',
    copied: '✓ Copiado',
    chatTitle: 'Chat IA',
    chatPlaceholder: 'Pergunte sobre o vídeo. O chat usa a transcrição e o texto final atual.',
    chatInput: 'Ex: melhore o gancho inicial, deixe mais executivo, responda uma dúvida sobre o vídeo…',
    btnSend: 'Perguntar à IA',
    loading: 'Processando…',
    loadingTranscript: 'Buscando transcrição…',
    loadingAnalysis: 'Analisando com IA…',
    loadingChat: 'Consultando a IA…',
    loadingImprove: 'Melhorando texto final…',
    errOllama: 'Não foi possível conectar ao Ollama.',
    errTranscript: 'Não foi possível obter a transcrição.',
    errModel: 'Erro carregando modelos.',
    errChat: 'Erro no chat. Tente novamente.',
    errInvalidConnection: 'Defina um host e porta válidos, ou um endpoint completo válido.',
    connSuccess: '✓ Conectado ao Ollama',
    connFail: '✗ Não foi possível conectar',
    noTranscript: 'Busque a transcrição primeiro.',
    noModel: 'Selecione um modelo.',
    noQuestion: 'Digite uma pergunta para a IA.',
    fetchSuccess: '✓ Transcrição obtida',
    improveSuccess: '✓ Texto final melhorado',
    chatReady: '✓ Resposta pronta',
    modelCount: (n) => `${n} modelo${n !== 1 ? 's' : ''} disponíve${n !== 1 ? 'is' : 'l'}`,
    metaTranscriptChars: 'Transcrição',
    metaOutput: 'Saída',
    assistantLabel: 'IA',
    userLabel: 'Você',
    btnInstall: 'Instalar app',
    btnInstalled: 'App instalada',
    installUnavailable: 'La instalación no está disponible en este contexto.',
    installAccepted: '✓ Instalación aceptada',
    installDismissed: 'Instalación cancelada',
    installManualHint: 'Si no aparece el diálogo, usa el menú del navegador para instalar la app.',
    runtimeBannerTitle: 'App web instalável',
    runtimeBannerRemote: 'Instale a PWA para abrir a interface mais rápido e usá-la offline. Para transcrição confiável e integração completa com o Ollama, execute o runtime web local ou aponte o backend de transcrição para ele.',
    runtimeBannerLocal: 'Você está usando o runtime local completo. Instale a PWA para abrir mais rápido enquanto transcrição e Ollama continuam resolvidos no seu ambiente local.',
    runtimePillRemote: 'Modo interface / Pages',
    runtimePillLocal: 'Modo local completo',
    runtimeInstalledIcon: '✅',
    runtimeBannerInstalledLocal: 'Estás usando la app ya instalada con el runtime local completo. Transcript y Ollama se resuelven en tu entorno local.',
    runtimeBannerInstalledRemote: 'Estás usando la app ya instalada. La interfaz puede abrir offline, pero para transcript confiable y Ollama completo conviene usar el runtime web local.',
  },
  fr: {
    settingsTitle: 'Connexion Ollama',
    settingsHint: 'Configurez Ollama par hôte/port ou via un endpoint complet. Un override explicite a priorité.',
    settingsUrl: 'Endpoint complet (override)',
    settingsProxy: 'Proxy CORS (transcripts)',
    settingsTranscriptBackend: 'Backend transcript (optionnel)',
    settingsHostPort: 'Hôte et port',
    settingsEffectiveUrl: 'URL effective',
    settingsTest: 'Tester',
    settingsSave: 'Enregistrer',
    urlLabel: 'URL OU ID YOUTUBE',
    languagesLabel: 'LANGUES DU TRANSCRIPT',
    modelLabel: 'MODÈLE',
    styleLabel: 'STYLE DE SORTIE',
    styleChat: 'Chat prêt à coller',
    styleExec: 'Résumé exécutif',
    styleBullets: 'Points directs',
    btnFetch: 'Récupérer le transcript',
    btnAnalyze: 'Générer le résumé',
    btnImprove: 'Améliorer le texte final',
    metaTitle: 'Métadonnées',
    summaryTitle: 'Résumé',
    keyPointsTitle: 'Points importants',
    transcriptTitle: 'Transcript',
    shareTitle: 'Texte prêt à coller',
    btnShow: 'Afficher',
    btnHide: 'Masquer',
    btnCopy: 'Copier',
    copied: '✓ Copié',
    chatTitle: 'Chat IA',
    chatPlaceholder: 'Posez une question sur la vidéo. Le chat utilise le transcript et le texte final actuel.',
    chatInput: 'Ex : améliore l’accroche, rends-le plus exécutif, réponds à une question sur la vidéo…',
    btnSend: 'Demander à l’IA',
    loading: 'Traitement…',
    loadingTranscript: 'Récupération du transcript…',
    loadingAnalysis: 'Analyse IA…',
    loadingChat: 'Consultation de l’IA…',
    loadingImprove: 'Amélioration du texte final…',
    errOllama: 'Impossible de se connecter à Ollama.',
    errTranscript: 'Impossible de récupérer le transcript.',
    errModel: 'Erreur de chargement des modèles.',
    errChat: 'Erreur du chat.',
    errInvalidConnection: 'Définissez un hôte et un port valides, ou un endpoint complet valide.',
    connSuccess: '✓ Connecté à Ollama',
    connFail: '✗ Connexion impossible',
    noTranscript: 'Récupérez d’abord le transcript.',
    noModel: 'Sélectionnez un modèle.',
    noQuestion: 'Tapez une question pour l’IA.',
    fetchSuccess: '✓ Transcript récupéré',
    improveSuccess: '✓ Texte final amélioré',
    chatReady: '✓ Réponse prête',
    modelCount: (n) => `${n} modèle${n !== 1 ? 's' : ''} disponible${n !== 1 ? 's' : ''}`,
    metaTranscriptChars: 'Transcript',
    metaOutput: 'Sortie',
    assistantLabel: 'IA',
    userLabel: 'Vous',
    btnInstall: 'Installer l’app',
    btnInstalled: 'App installée',
    installUnavailable: 'L’installation n’est pas disponible dans ce contexte.',
    installAccepted: '✓ Installation acceptée',
    installDismissed: 'Installation annulée',
    installManualHint: 'Si aucun dialogue n’apparaît, utilisez le menu du navigateur pour installer l’app.',
    runtimeBannerTitle: 'Application web installable',
    runtimeBannerRemote: 'Installez la PWA pour ouvrir l’interface plus vite et l’utiliser hors ligne. Pour des transcriptions fiables et une intégration complète avec Ollama, lancez le runtime web local ou pointez le backend transcript vers celui-ci.',
    runtimeBannerLocal: 'Vous utilisez le runtime local complet. Installez la PWA pour un lancement rapide pendant que les transcriptions et Ollama restent gérés localement.',
    runtimePillRemote: 'Mode interface / Pages',
    runtimePillLocal: 'Mode local complet',
    runtimeInstalledIcon: '✅',
    runtimeBannerInstalledLocal: 'Vous utilisez déjà l’application installée avec le runtime local complet. Les transcriptions et Ollama restent gérés dans votre environnement local.',
    runtimeBannerInstalledRemote: 'Vous utilisez déjà l’application installée. L’interface peut s’ouvrir hors ligne, mais pour des transcriptions fiables et une intégration complète avec Ollama, utilisez le runtime web local.',
  },
  de: {
    settingsTitle: 'Ollama-Verbindung',
    settingsHint: 'Konfigurieren Sie Ollama per Host/Port oder über einen kompletten Endpoint. Ein Override hat Vorrang.',
    settingsUrl: 'Voller Endpoint (Override)',
    settingsProxy: 'CORS-Proxy (Transkripte)',
    settingsTranscriptBackend: 'Transcript-Backend (optional)',
    settingsHostPort: 'Host und Port',
    settingsEffectiveUrl: 'Effektive URL',
    settingsTest: 'Testen',
    settingsSave: 'Speichern',
    urlLabel: 'YOUTUBE-URL ODER ID',
    languagesLabel: 'TRANSKRIPT-SPRACHEN',
    modelLabel: 'MODELL',
    styleLabel: 'AUSGABESTIL',
    styleChat: 'Chat zum Einfügen',
    styleExec: 'Executive-Zusammenfassung',
    styleBullets: 'Direkte Stichpunkte',
    btnFetch: 'Transkript holen',
    btnAnalyze: 'Zusammenfassung erzeugen',
    btnImprove: 'Finalen Text verbessern',
    metaTitle: 'Metadaten',
    summaryTitle: 'Zusammenfassung',
    keyPointsTitle: 'Wichtige Punkte',
    transcriptTitle: 'Transkript',
    shareTitle: 'Text zum Einfügen',
    btnShow: 'Anzeigen',
    btnHide: 'Ausblenden',
    btnCopy: 'Kopieren',
    copied: '✓ Kopiert',
    chatTitle: 'KI-Chat',
    chatPlaceholder: 'Fragen Sie etwas zum Video. Der Chat nutzt Transkript und aktuellen Endtext.',
    chatInput: 'Z. B.: verbessere den Einstieg, mach ihn exekutiver, beantworte eine Frage zum Video…',
    btnSend: 'KI fragen',
    loading: 'Verarbeitung…',
    loadingTranscript: 'Transkript wird geladen…',
    loadingAnalysis: 'KI analysiert…',
    loadingChat: 'KI wird konsultiert…',
    loadingImprove: 'Finaler Text wird verbessert…',
    errOllama: 'Verbindung zu Ollama fehlgeschlagen.',
    errTranscript: 'Transkript konnte nicht geladen werden.',
    errModel: 'Fehler beim Laden der Modelle.',
    errChat: 'Chat-Fehler.',
    errInvalidConnection: 'Geben Sie einen gültigen Host und Port oder einen gültigen Endpoint an.',
    connSuccess: '✓ Mit Ollama verbunden',
    connFail: '✗ Verbindung fehlgeschlagen',
    noTranscript: 'Zuerst das Transkript holen.',
    noModel: 'Wählen Sie ein Modell aus.',
    noQuestion: 'Geben Sie eine Frage für die KI ein.',
    fetchSuccess: '✓ Transkript geladen',
    improveSuccess: '✓ Finaler Text verbessert',
    chatReady: '✓ Antwort bereit',
    modelCount: (n) => `${n} Modell${n !== 1 ? 'e' : ''} verfügbar`,
    metaTranscriptChars: 'Transkript',
    metaOutput: 'Ausgabe',
    assistantLabel: 'KI',
    userLabel: 'Sie',
    btnInstall: 'App installieren',
    btnInstalled: 'App installiert',
    installUnavailable: 'Die Installation ist in diesem Kontext nicht verfügbar.',
    installAccepted: '✓ Installation bestätigt',
    installDismissed: 'Installation abgebrochen',
    installManualHint: 'Wenn kein Dialog erscheint, installiere die App über das Browser-Menü.',
    runtimeBannerTitle: 'Installierbare Web-App',
    runtimeBannerRemote: 'Installiere die PWA für schnellen Zugriff und eine offline verfügbare Oberfläche. Für zuverlässige Transkripte und vollständige Ollama-Integration nutze den lokalen Web-Runtime oder verweise das Transcript-Backend darauf.',
    runtimeBannerLocal: 'Du nutzt den vollständigen lokalen Runtime-Modus. Installiere die PWA für schnellen Start, während Transkript und Ollama lokal aufgelöst werden.',
    runtimePillRemote: 'Interface- / Pages-Modus',
    runtimePillLocal: 'Voller lokaler Modus',
    runtimeInstalledIcon: '✅',
    runtimeBannerInstalledLocal: 'Du verwendest die bereits installierte App mit vollständigem lokalem Runtime. Transkript und Ollama bleiben in deiner lokalen Umgebung.',
    runtimeBannerInstalledRemote: 'Du verwendest die bereits installierte App. Die Oberfläche kann offline geöffnet werden, aber für zuverlässige Transkripte und volle Ollama-Integration ist der lokale Web-Runtime besser.',
  },
};

for (const code of ['ja', 'zh-Hans', 'ru', 'ar', 'hi']) {
  i18n[code] = { ...i18n.en };
}

const State = {
  lang: localStorage.getItem('cliptube-app-lang') || 'es',
  ollamaHost: localStorage.getItem('cliptube-ollama-host') || '127.0.0.1',
  ollamaPort: localStorage.getItem('cliptube-ollama-port') || '11434',
  ollamaEndpointOverride: localStorage.getItem('cliptube-ollama-endpoint-override') || '',
  corsProxy: localStorage.getItem('cliptube-cors-proxy') || 'https://corsproxy.io/?',
  transcriptBackend: localStorage.getItem('cliptube-transcript-backend') || '',
  selectedModel: localStorage.getItem('cliptube-model-name') || '',
  transcriptLanguages: localStorage.getItem('cliptube-transcript-languages') || '',
  models: [],
  outputStyle: Number(localStorage.getItem('cliptube-output-style') || 0),
  transcript: null,
  summary: null,
  shareText: '',
  chatHistory: [],
  busy: false,
};

let deferredInstallPrompt = null;

const $ = (sel) => document.querySelector(sel);
const $$ = (sel) => document.querySelectorAll(sel);

function t(key) {
  const dict = i18n[State.lang] || i18n.en;
  const val = dict[key];
  if (typeof val === 'function') return val;
  return val || i18n.en[key] || key;
}

function defaultTranscriptLanguages(lang) {
  return TRANSCRIPT_LANGUAGE_DEFAULTS[lang] || 'en,es';
}

function normalizeUiLanguage(lang) {
  return LANGUAGE_OPTIONS.some(([code]) => code === lang) ? lang : 'en';
}

function renderLanguageOptions() {
  const select = $('#lang-select');
  const current = State.lang;
  select.innerHTML = '';
  LANGUAGE_OPTIONS.forEach(([code, label]) => {
    const option = document.createElement('option');
    option.value = code;
    option.textContent = label;
    select.appendChild(option);
  });
  select.value = current;
}

function applyLanguage(lang) {
  const previous = State.lang;
  State.lang = normalizeUiLanguage(lang);
  localStorage.setItem('cliptube-app-lang', State.lang);
  document.documentElement.lang = State.lang;
  renderLanguageOptions();

  $$('[data-i18n]').forEach((el) => {
    const val = t(el.dataset.i18n);
    if (typeof val === 'string') el.textContent = val;
  });

  $$('[data-i18n-placeholder]').forEach((el) => {
    const val = t(el.dataset.i18nPlaceholder);
    if (typeof val === 'string') el.placeholder = val;
  });

  if (!State.transcriptLanguages || State.transcriptLanguages === defaultTranscriptLanguages(previous)) {
    State.transcriptLanguages = defaultTranscriptLanguages(State.lang);
  }
  $('#transcript-languages').value = State.transcriptLanguages;
  $('#chat-style').value = String(State.outputStyle);
  syncStyleButtons();
  $('#chat-model-name').textContent = State.selectedModel || '';
  updateEffectiveUrlLabel();
  updateRuntimeBanner();
  renderChatHistory();
  if (State.summary) displayResults(State.summary);
}

function loadSettings() {
  $('#ollama-host').value = State.ollamaHost;
  $('#ollama-port').value = State.ollamaPort;
  $('#ollama-url').value = State.ollamaEndpointOverride;
  $('#cors-proxy').value = State.corsProxy;
  $('#transcript-backend').value = State.transcriptBackend;
  if (!State.transcriptLanguages) State.transcriptLanguages = defaultTranscriptLanguages(State.lang);
  $('#transcript-languages').value = State.transcriptLanguages;
  updateEffectiveUrlLabel();
  updateRuntimeBanner();
  void updateInstallButtonState();
}

function effectiveOllamaBaseUrl() {
  const override = $('#ollama-url').value.trim();
  if (override) {
    if (/^https?:\/\//.test(override)) return override.replace(/\/api\/chat\/?$/, '').replace(/\/$/, '');
    throw new Error(t('errInvalidConnection'));
  }
  const host = $('#ollama-host').value.trim();
  const port = $('#ollama-port').value.trim();
  if (!host || !/^\d+$/.test(port) || Number(port) < 1 || Number(port) > 65535) {
    throw new Error(t('errInvalidConnection'));
  }
  const normalizedHost = /^https?:\/\//.test(host) ? host : `http://${host}`;
  return `${normalizedHost.replace(/\/$/, '')}:${port}`;
}

function effectiveOllamaChatUrl() {
  return `${effectiveOllamaBaseUrl().replace(/\/$/, '')}/api/chat`;
}

function effectiveTranscriptBackendBase() {
  const configured = ($('#transcript-backend')?.value || '').trim() || State.transcriptBackend;
  return configured ? configured.replace(/\/$/, '') : window.location.origin;
}

function updateEffectiveUrlLabel() {
  const el = $('#effective-url');
  try {
    el.textContent = effectiveOllamaBaseUrl();
    el.style.color = 'var(--cyan)';
  } catch {
    el.textContent = t('errInvalidConnection');
    el.style.color = 'var(--red)';
  }
}

function saveSettings() {
  try {
    const effective = effectiveOllamaBaseUrl();
    State.ollamaHost = $('#ollama-host').value.trim();
    State.ollamaPort = $('#ollama-port').value.trim();
    State.ollamaEndpointOverride = $('#ollama-url').value.trim();
    State.corsProxy = $('#cors-proxy').value.trim();
    State.transcriptBackend = $('#transcript-backend').value.trim();
    State.transcriptLanguages = $('#transcript-languages').value.trim() || defaultTranscriptLanguages(State.lang);

    localStorage.setItem('cliptube-ollama-host', State.ollamaHost);
    localStorage.setItem('cliptube-ollama-port', State.ollamaPort);
    localStorage.setItem('cliptube-ollama-endpoint-override', State.ollamaEndpointOverride);
    localStorage.setItem('cliptube-cors-proxy', State.corsProxy);
    localStorage.setItem('cliptube-transcript-backend', State.transcriptBackend);
    localStorage.setItem('cliptube-transcript-languages', State.transcriptLanguages);

    $('#connection-result').textContent = `${t('settingsEffectiveUrl')}: ${effective}`;
    updateRuntimeBanner();
    $('#connection-result').style.color = 'var(--cyan)';
    loadModels();
    return true;
  } catch (error) {
    $('#connection-result').textContent = error.message;
    $('#connection-result').style.color = 'var(--red)';
    updateStatusDot(false);
    return false;
  }
}

async function testConnection() {
  const resultEl = $('#connection-result');
  resultEl.textContent = '…';
  resultEl.className = 'connection-result';
  try {
    const res = await fetch(`${effectiveOllamaBaseUrl()}/api/tags`, { signal: AbortSignal.timeout(8000) });
    if (!res.ok) throw new Error(res.statusText);
    const data = await res.json();
    const count = data.models?.length || 0;
    resultEl.textContent = `${t('connSuccess')} — ${t('modelCount')(count)}`;
    resultEl.style.color = 'var(--green)';
    updateStatusDot(true);
  } catch (error) {
    resultEl.textContent = error.message || t('connFail');
    resultEl.style.color = 'var(--red)';
    updateStatusDot(false);
  }
}

function updateStatusDot(connected) {
  const dot = $('#ollama-status');
  dot.classList.toggle('connected', connected);
  dot.title = connected ? t('connSuccess') : t('connFail');
}

async function loadModels() {
  const select = $('#model-select');
  select.innerHTML = `<option value="">${t('errModel')}</option>`;
  State.models = [];
  try {
    const res = await fetch(`${effectiveOllamaBaseUrl()}/api/tags`, { signal: AbortSignal.timeout(8000) });
    if (!res.ok) throw new Error(res.statusText);
    const data = await res.json();
    State.models = (data.models || []).map((m) => ({
      name: m.name,
      size: m.size,
      family: m.details?.family || '',
      params: m.details?.parameter_size || '',
      quant: m.details?.quantization_level || '',
      isCloud: !!m.remote_model,
    }));

    select.innerHTML = '';
    State.models.forEach((m) => {
      const opt = document.createElement('option');
      opt.value = m.name;
      const tag = m.isCloud ? ' ☁️' : '';
      const info = m.params ? ` (${m.params} · ${m.quant})` : '';
      opt.textContent = `${m.name}${tag}${info}`;
      select.appendChild(opt);
    });

    if (State.selectedModel && State.models.some((m) => m.name === State.selectedModel)) {
      select.value = State.selectedModel;
    } else if (State.models.length > 0) {
      State.selectedModel = State.models[0].name;
      localStorage.setItem('cliptube-model-name', State.selectedModel);
      select.value = State.selectedModel;
    }

    $('#chat-model-name').textContent = State.selectedModel || '';
    $('#btn-analyze').disabled = !State.transcript || !State.selectedModel;
    updateStatusDot(true);
  } catch (error) {
    select.innerHTML = `<option value="">${error.message || t('errOllama')}</option>`;
    updateStatusDot(false);
  }
}

function extractVideoId(input) {
  input = input.trim();
  if (/^[A-Za-z0-9_-]{11}$/.test(input)) return input;
  try {
    const url = new URL(input);
    if (url.hostname === 'youtu.be') return url.pathname.slice(1);
    if (url.pathname === '/watch') return url.searchParams.get('v');
    if (/^\/(shorts|embed|live)\//.test(url.pathname)) return url.pathname.split('/')[2];
  } catch {}
  return null;
}

function preferredTranscriptLanguages() {
  return ($('#transcript-languages').value.trim() || defaultTranscriptLanguages(State.lang))
    .split(',')
    .map((part) => part.trim().toLowerCase())
    .filter(Boolean);
}

function extractInnertubeApiKey(html) {
  const patterns = [
    /"INNERTUBE_API_KEY"\s*:\s*"([A-Za-z0-9_-]+)"/,
    /ytcfg\.set\(\{[^}]*"INNERTUBE_API_KEY"\s*:\s*"([A-Za-z0-9_-]+)"/s,
  ];
  for (const pattern of patterns) {
    const match = html.match(pattern);
    if (match?.[1]) return match[1];
  }
  return null;
}

function extractCaptionTracksFromPlayer(player) {
  return player?.captions?.playerCaptionsTracklistRenderer?.captionTracks || [];
}

async function fetchPlayerResponseFromInnertube(videoId, apiKey) {
  const apiUrl = `https://www.youtube.com/youtubei/v1/player?key=${apiKey}`;
  const proxiedApiUrl = `${State.corsProxy}${encodeURIComponent(apiUrl)}`;
  const payload = {
    context: {
      client: {
        clientName: 'ANDROID',
        clientVersion: '20.10.38',
      },
    },
    videoId,
  };

  const res = await fetch(proxiedApiUrl, {
    method: 'POST',
    headers: {
      'content-type': 'application/json',
    },
    body: JSON.stringify(payload),
    signal: AbortSignal.timeout(30000),
  });

  if (!res.ok) throw new Error(`Innertube HTTP ${res.status}`);
  return res.json();
}

async function fetchTranscript() {
  const urlInput = $('#youtube-url').value.trim();
  const videoId = extractVideoId(urlInput);
  if (!videoId) {
    setStatus('fetch-status', `${t('errTranscript')} URL/ID inválido.`, 'error');
    return;
  }
  showLoading(t('loadingTranscript'));
  setStatus('fetch-status', '', '');
  try {
    const transcriptBase = effectiveTranscriptBackendBase();
    if (transcriptBase) {
      try {
        const transcriptUrl = new URL(`${transcriptBase}/api/transcript`);
        transcriptUrl.searchParams.set('input', urlInput || videoId);
        transcriptUrl.searchParams.set('languages', preferredTranscriptLanguages().join(','));
        const apiRes = await fetch(transcriptUrl.toString(), { signal: AbortSignal.timeout(30000) });
        if (apiRes.ok) {
          const apiData = await apiRes.json();
          State.transcript = {
            videoId: apiData.videoId || videoId,
            sourceUrl: apiData.sourceUrl || `https://www.youtube.com/watch?v=${videoId}`,
            language: apiData.languageLabel || 'unknown',
            isGenerated: !!apiData.isGenerated,
            fullText: apiData.fullText || '',
          };
          if (!State.transcript.fullText.trim()) throw new Error('Empty transcript');
          State.chatHistory = [{ role: 'assistant', content: t('chatPlaceholder') }];
          renderChatHistory();
          const lines = State.transcript.fullText.split('\n').filter(Boolean).length;
          setStatus('fetch-status', `${t('fetchSuccess')} (${lines} líneas, ${State.transcript.language})`, 'success');
          $('#btn-analyze').disabled = !State.selectedModel;
          return;
        }
      } catch (backendError) {
        console.warn('Transcript backend unavailable, falling back to browser mode:', backendError);
      }
    }

    const watchUrl = `https://www.youtube.com/watch?v=${videoId}`;
    const proxyUrl = `${State.corsProxy}${encodeURIComponent(watchUrl)}`;
    const pageRes = await fetch(proxyUrl, { signal: AbortSignal.timeout(30000) });
    if (!pageRes.ok) throw new Error(`HTTP ${pageRes.status}`);
    const html = await pageRes.text();

    const playerMatch = html.match(/ytInitialPlayerResponse\s*=\s*(\{.*?\});/s);
    let player = playerMatch ? JSON.parse(playerMatch[1]) : null;
    let captionTracks = extractCaptionTracksFromPlayer(player);

    if (captionTracks.length === 0) {
      const apiKey = extractInnertubeApiKey(html);
      if (!apiKey) throw new Error('No caption tracks found (missing INNERTUBE_API_KEY)');
      player = await fetchPlayerResponseFromInnertube(videoId, apiKey);
      captionTracks = extractCaptionTracksFromPlayer(player);
    }

    if (captionTracks.length === 0) {
      const status = player?.playabilityStatus?.status || 'UNKNOWN';
      const reason = player?.playabilityStatus?.reason || 'No caption tracks found';
      throw new Error(`${reason} [${status}]`);
    }

    const preferred = preferredTranscriptLanguages();
    let track = null;
    for (const lang of preferred) {
      track = captionTracks.find((t) => t.languageCode?.startsWith(lang) && t.kind !== 'asr');
      if (track) break;
    }
    if (!track) {
      for (const lang of preferred) {
        track = captionTracks.find((t) => t.languageCode?.startsWith(lang));
        if (track) break;
      }
    }
    track = track || captionTracks.find((t) => t.kind !== 'asr') || captionTracks[0];

    const captionUrl = track.baseUrl.replace('&fmt=srv3', '');
    const captionProxyUrl = `${State.corsProxy}${encodeURIComponent(captionUrl)}`;
    const xmlRes = await fetch(captionProxyUrl, { signal: AbortSignal.timeout(20000) });
    if (!xmlRes.ok) throw new Error(`Caption HTTP ${xmlRes.status}`);
    const xml = await xmlRes.text();

    const parser = new DOMParser();
    const doc = parser.parseFromString(xml, 'text/xml');
    const textNodes = doc.querySelectorAll('text');
    const lines = [];
    textNodes.forEach((node) => {
      const text = node.textContent.replace(/<[^>]+>/g, '').trim();
      if (text) lines.push(text);
    });
    if (lines.length === 0) throw new Error('Empty transcript');

    const fullText = lines.join('\n');
    State.transcript = {
      videoId,
      sourceUrl: `https://www.youtube.com/watch?v=${videoId}`,
      language: track.languageCode || track.name?.simpleText || 'unknown',
      isGenerated: track.kind === 'asr',
      fullText,
    };
    State.chatHistory = [{ role: 'assistant', content: t('chatPlaceholder') }];
    renderChatHistory();
    setStatus('fetch-status', `${t('fetchSuccess')} (${lines.length} líneas, ${State.transcript.language})`, 'success');
    $('#btn-analyze').disabled = !State.selectedModel;
  } catch (err) {
    console.error('Transcript error:', err);
    const detail = String(err?.message || err || '');
    const friendly = /No caption tracks found|Failed to fetch|Innertube HTTP 403|Cloudflare/i.test(detail)
      ? `${t('errTranscript')} El navegador no pudo resolver este video con el proxy público. Configura un backend de transcript local en Ajustes.`
      : `${t('errTranscript')} ${detail}`;
    setStatus('fetch-status', friendly, 'error');
    showToast(friendly);
  } finally {
    hideLoading();
  }
}

function languageNameForPrompt() {
  const map = {
    es: 'español', en: 'English', pt: 'português', fr: 'français', de: 'Deutsch',
    ja: '日本語', 'zh-Hans': '简体中文', ru: 'Русский', ar: 'العربية', hi: 'हिन्दी',
  };
  return map[State.lang] || 'English';
}

function getOutputStyleName(style) {
  return style === 1 ? 'executive' : style === 2 ? 'bullets' : 'chat';
}

function getAnalyzeSystemPrompt(style) {
  const styleInstructions = {
    chat: "'chat_text' must be punchy, concise, and ready to paste in chat. Keep it faithful to the video. Include URL, a strong quick summary, and important points.",
    executive: "'chat_text' must be executive, clear, and suitable for sharing with a team or manager. Include URL, context, a concise summary, and key points.",
    bullets: "'chat_text' must be ultra direct, mostly short bullets, including URL and actionable conclusions.",
  };

  return `You are an expert video analysis assistant. Respond ONLY with valid JSON using this exact structure:\n\n{\n  \"summary\": \"Clear summary of the video content (3-5 sentences)\",\n  \"key_points\": [\"Key point 1\", \"Key point 2\", \"Key point 3\"],\n  \"chat_text\": \"Text ready to share\",\n  \"status\": \"completed\"\n}\n\n${styleInstructions[getOutputStyleName(style)]}\n\nReply in ${languageNameForPrompt()}. JSON only, no markdown, no extra explanation.`;
}

function buildShareText(parsed) {
  const bulletPrefix = State.lang === 'zh-Hans' || State.lang === 'ja' ? '• ' : '• ';
  const keyPointsText = parsed.keyPoints.map((p) => `${bulletPrefix}${p}`).join('\n');
  return `${(parsed.chatText || '').trim()}\n\n\n${t('summaryTitle')}\n${(parsed.summary || '').trim()}\n\n\n${t('keyPointsTitle')}\n${keyPointsText.trim()}\n\n\n${State.transcript?.sourceUrl || ''}`.trim();
}

async function analyzeVideo() {
  if (!State.transcript) return showToast(t('noTranscript'));
  if (!State.selectedModel) return showToast(t('noModel'));
  showLoading(t('loadingAnalysis'));
  hideResults();
  try {
    const transcriptText = State.transcript.fullText.slice(0, 90000);
    const response = await callOllama(State.selectedModel, [
      { role: 'system', content: getAnalyzeSystemPrompt(State.outputStyle) },
      { role: 'user', content: `Video transcript (${State.transcript.sourceUrl}):\n\n${transcriptText}` },
    ], { json: true });

    const parsed = parseAiResponse(response);
    State.summary = parsed;
    State.shareText = buildShareText(parsed);
    displayResults(parsed);
  } catch (err) {
    console.error('Analysis error:', err);
    showToast(`${t('errOllama')} ${err.message}`);
  } finally {
    hideLoading();
  }
}

async function sendChat() {
  const input = $('#chat-input');
  const question = input.value.trim();
  if (!question) return showToast(t('noQuestion'));
  if (!State.transcript) return showToast(t('noTranscript'));
  if (!State.selectedModel) return showToast(t('noModel'));

  input.value = '';
  State.chatHistory.push({ role: 'user', content: question });
  State.chatHistory.push({ role: 'assistant', content: '…', streaming: true });
  renderChatHistory();
  showLoading(t('loadingChat'));

  const messages = [
    {
      role: 'system',
      content: `You answer questions about a YouTube video using its transcript and current share-ready text. Reply in ${languageNameForPrompt()}. Be concise, useful, and grounded in the provided content.\n\nURL: ${State.transcript.sourceUrl}\n\nCurrent share text:\n${State.shareText || '-'}\n\nTranscript:\n${State.transcript.fullText.slice(0, 60000)}`,
    },
    ...State.chatHistory.filter((m) => !m.streaming).map((m) => ({ role: m.role === 'assistant' ? 'assistant' : 'user', content: m.content })),
  ];

  try {
    const response = await callOllama(State.selectedModel, messages);
    State.chatHistory[State.chatHistory.length - 1] = { role: 'assistant', content: response };
    renderChatHistory();
    showToast(t('chatReady'));
  } catch (err) {
    State.chatHistory[State.chatHistory.length - 1] = { role: 'assistant', content: `❌ ${t('errChat')} ${err.message}` };
    renderChatHistory();
  } finally {
    hideLoading();
  }
}

async function improveFinalText() {
  if (!State.transcript) return showToast(t('noTranscript'));
  if (!State.selectedModel) return showToast(t('noModel'));
  if (!State.shareText) return showToast(t('noTranscript'));

  const userNote = $('#chat-input').value.trim();
  showLoading(t('loadingImprove'));
  State.chatHistory.push({ role: 'assistant', content: '…', streaming: true });
  renderChatHistory();

  const prompt = `Improve the existing share-ready text while preserving the structure: intro text, ${t('summaryTitle')}, ${t('keyPointsTitle')}, and final URL. Keep it faithful to the transcript and reply in ${languageNameForPrompt()}.\n\nCurrent share text:\n${State.shareText}\n\nUser extra instructions:\n${userNote || '-'}\n\nTranscript:\n${State.transcript.fullText.slice(0, 60000)}`;

  try {
    const response = await callOllama(State.selectedModel, [
      { role: 'system', content: 'You improve existing share-ready text without inventing facts. Return plain text only.' },
      { role: 'user', content: prompt },
    ]);
    State.shareText = response.trim();
    $('#result-share').textContent = State.shareText;
    State.chatHistory[State.chatHistory.length - 1] = { role: 'assistant', content: response };
    renderChatHistory();
    showToast(t('improveSuccess'));
  } catch (err) {
    State.chatHistory[State.chatHistory.length - 1] = { role: 'assistant', content: `❌ ${t('errChat')} ${err.message}` };
    renderChatHistory();
  } finally {
    hideLoading();
  }
}

async function callOllama(model, messages, opts = {}) {
  const body = {
    model,
    messages,
    stream: false,
    options: {
      temperature: 0.7,
      num_predict: 4096,
    },
  };
  if (opts.json) body.format = 'json';

  const res = await fetch(effectiveOllamaChatUrl(), {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
    signal: AbortSignal.timeout(180000),
  });
  if (!res.ok) {
    const errText = await res.text().catch(() => '');
    throw new Error(`HTTP ${res.status}: ${errText.slice(0, 200)}`);
  }
  const data = await res.json();
  return data.message?.content || '';
}

function parseAiResponse(raw) {
  const attempts = [raw, raw.replace(/```json\s*/gi, '').replace(/```/g, '').trim()];
  for (const candidate of attempts) {
    try {
      const obj = JSON.parse(candidate);
      return {
        summary: obj.summary || '',
        keyPoints: obj.key_points || [],
        chatText: obj.chat_text || '',
        status: obj.status || 'completed',
      };
    } catch {}
    const start = candidate.indexOf('{');
    const end = candidate.lastIndexOf('}');
    if (start !== -1 && end > start) {
      try {
        const obj = JSON.parse(candidate.slice(start, end + 1));
        return {
          summary: obj.summary || '',
          keyPoints: obj.key_points || [],
          chatText: obj.chat_text || '',
          status: obj.status || 'completed',
        };
      } catch {}
    }
  }
  return { summary: raw.slice(0, 500), keyPoints: [], chatText: raw, status: 'parse_error' };
}

function setStatus(id, text, type) {
  const el = $(`#${id}`);
  el.textContent = text;
  el.className = `status-text ${type || ''}`;
}

function showLoading(text) {
  State.busy = true;
  $('#loading-text').textContent = text || t('loading');
  $('#loading').classList.remove('hidden');
}

function hideLoading() {
  State.busy = false;
  $('#loading').classList.add('hidden');
}

function hideResults() {
  $('#results').classList.add('hidden');
}

function displayResults(parsed) {
  $('#results').classList.remove('hidden');
  $('#result-meta').textContent =
    `URL: ${State.transcript.sourceUrl}\nID: ${State.transcript.videoId}\n${t('metaOutput')}: ${$(`.style-btn[data-style="${State.outputStyle}"]`).textContent}\n${t('metaTranscriptChars')}: ${State.transcript.fullText.length} chars\nModel: ${State.selectedModel}\nOllama: ${safeEffectiveUrl()}`;
  $('#result-summary').textContent = parsed.summary;
  const ul = $('#result-keypoints');
  ul.innerHTML = '';
  if (parsed.keyPoints.length === 0) {
    ul.innerHTML = `<li style="color:var(--muted)">—</li>`;
  } else {
    parsed.keyPoints.forEach((p) => {
      const li = document.createElement('li');
      li.textContent = p;
      ul.appendChild(li);
    });
  }
  $('#result-transcript').textContent = State.transcript.fullText;
  $('#result-share').textContent = State.shareText;
}

function safeEffectiveUrl() {
  try { return effectiveOllamaBaseUrl(); } catch { return '—'; }
}

function renderChatHistory() {
  const container = $('#chat-messages');
  container.innerHTML = '';
  if (!State.chatHistory.length) {
    const placeholder = document.createElement('div');
    placeholder.className = 'chat-placeholder';
    placeholder.textContent = t('chatPlaceholder');
    container.appendChild(placeholder);
    return;
  }
  State.chatHistory.forEach((message) => {
    const role = message.role === 'user' ? 'user' : 'assistant';
    const msg = document.createElement('div');
    msg.className = `chat-msg ${role}`;
    msg.innerHTML = `
      <span class="chat-role">${role === 'user' ? t('userLabel') : t('assistantLabel')}</span>
      <div class="chat-bubble">${escapeHtml(message.content)}</div>
    `;
    container.appendChild(msg);
  });
  container.scrollTop = container.scrollHeight;
}

function escapeHtml(text) {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
}

function showToast(msg) {
  const toast = $('#toast');
  toast.textContent = msg;
  toast.classList.add('show');
  toast.classList.remove('hidden');
  setTimeout(() => toast.classList.remove('show'), 4000);
}

function copyShareText() {
  const text = $('#result-share').textContent;
  navigator.clipboard.writeText(text).then(() => {
    const fb = $('#copy-feedback');
    fb.textContent = t('copied');
    setTimeout(() => { fb.textContent = ''; }, 2500);
  }).catch(() => {
    const el = $('#result-share');
    const range = document.createRange();
    range.selectNodeContents(el);
    window.getSelection().removeAllRanges();
    window.getSelection().addRange(range);
    document.execCommand('copy');
  });
}

function isLocalRuntimeMode() {
  const host = window.location.hostname;
  if (host === '127.0.0.1' || host === 'localhost') return true;
  const backend = ($('#transcript-backend')?.value || State.transcriptBackend || '').trim();
  return backend.includes('127.0.0.1') || backend.includes('localhost');
}

function isPwaInstalled() {
  return window.matchMedia('(display-mode: standalone)').matches || window.matchMedia('(display-mode: window-controls-overlay)').matches || navigator.standalone === true || localStorage.getItem('cliptube-pwa-installed') === '1';
}

async function detectInstalledRelatedApps() {
  if (typeof navigator.getInstalledRelatedApps !== 'function') return false;
  try {
    const apps = await navigator.getInstalledRelatedApps();
    return Array.isArray(apps) && apps.length > 0;
  } catch {
    return false;
  }
}

async function updateInstallButtonState() {
  const installBtn = $('#btn-install');
  if (!installBtn) return;
  const installed = isPwaInstalled() || await detectInstalledRelatedApps();
  if (installed) {
    localStorage.setItem('cliptube-pwa-installed', '1');
    installBtn.hidden = false;
    installBtn.disabled = true;
    installBtn.classList.add('is-installed');
    installBtn.textContent = t('btnInstalled') || t('btnInstall');
    installBtn.setAttribute('aria-disabled', 'true');
    installBtn.setAttribute('title', t('btnInstalled') || t('btnInstall'));
    return;
  }
  installBtn.disabled = !deferredInstallPrompt;
  installBtn.hidden = !deferredInstallPrompt;
  installBtn.classList.remove('is-installed');
  installBtn.textContent = t('btnInstall');
  installBtn.removeAttribute('aria-disabled');
  installBtn.removeAttribute('title');
}

function updateRuntimeBanner() {
  const icon = $('#runtime-banner-icon');
  const title = $('#runtime-banner-title');
  const pill = $('#runtime-mode-pill');
  const text = $('#runtime-banner-text');
  if (!pill || !text) return;
  const localMode = isLocalRuntimeMode();
  const installed = isPwaInstalled();
  if (icon) icon.textContent = installed ? (t('runtimeInstalledIcon') || '✅') : '📲';
  if (title) title.textContent = installed ? (t('btnInstalled') || t('runtimeBannerTitle')) : t('runtimeBannerTitle');
  pill.textContent = localMode ? t('runtimePillLocal') : t('runtimePillRemote');
  if (installed) {
    text.textContent = localMode
      ? (t('runtimeBannerInstalledLocal') || t('runtimeBannerLocal'))
      : (t('runtimeBannerInstalledRemote') || t('runtimeBannerRemote'));
  } else {
    text.textContent = localMode ? t('runtimeBannerLocal') : t('runtimeBannerRemote');
  }
}

function setupInstallPrompt() {
  const installBtn = $('#btn-install');
  if (!installBtn) return;
  void updateInstallButtonState();
  window.addEventListener('beforeinstallprompt', (event) => {
    event.preventDefault();
    deferredInstallPrompt = event;
    void updateInstallButtonState();
  });
  installBtn.addEventListener('click', async () => {
    if (isPwaInstalled()) {
      showToast(t('btnInstalled') || t('btnInstall'));
      return;
    }
    if (!deferredInstallPrompt) {
      showToast((t('installUnavailable') || t('btnInstall')) + ' ' + (t('installManualHint') || ''));
      return;
    }
    const promptEvent = deferredInstallPrompt;
    try {
      await promptEvent.prompt();
      const choice = await promptEvent.userChoice.catch(() => null);
      if (choice?.outcome === 'accepted') {
        localStorage.setItem('cliptube-pwa-installed', '1');
        showToast(t('installAccepted') || t('btnInstall'));
      } else {
        showToast(t('installDismissed') || t('installManualHint') || t('btnInstall'));
      }
    } finally {
      deferredInstallPrompt = null;
      void updateInstallButtonState();
    }
  });
  window.addEventListener('appinstalled', () => {
    localStorage.setItem('cliptube-pwa-installed', '1');
    deferredInstallPrompt = null;
    void updateInstallButtonState();
    showToast(t('btnInstalled') || t('btnInstall'));
  });
  const media = window.matchMedia('(display-mode: standalone)');
  const onModeChange = () => updateInstallButtonState();
  if (typeof media.addEventListener === 'function') media.addEventListener('change', onModeChange);
  else if (typeof media.addListener === 'function') media.addListener(onModeChange);
}

async function registerServiceWorker() {
  if (!('serviceWorker' in navigator)) return;
  try {
    await navigator.serviceWorker.register('./sw.js', { updateViaCache: 'none' });
    await navigator.serviceWorker.ready.catch(() => null);
    void updateInstallButtonState();
  } catch (error) {
    console.warn('Service worker registration failed:', error);
  }
}

function syncStyleButtons() {
  $$('.style-btn').forEach((btn) => {
    btn.classList.toggle('active', Number(btn.dataset.style) === State.outputStyle);
  });
}

function bindEvents() {
  $('#lang-select').addEventListener('change', (e) => {
    applyLanguage(e.target.value);
    loadModels();
  });
  $('#btn-settings').addEventListener('click', () => $('#settings-panel').classList.toggle('hidden'));
  $('#btn-test-connection').addEventListener('click', testConnection);
  $('#btn-save-settings').addEventListener('click', () => {
    if (saveSettings()) $('#settings-panel').classList.add('hidden');
  });
  $('#ollama-host').addEventListener('input', updateEffectiveUrlLabel);
  $('#ollama-port').addEventListener('input', updateEffectiveUrlLabel);
  $('#ollama-url').addEventListener('input', updateEffectiveUrlLabel);
  $('#transcript-backend').addEventListener('input', updateRuntimeBanner);
  $('#transcript-languages').addEventListener('change', (e) => {
    State.transcriptLanguages = e.target.value.trim();
    localStorage.setItem('cliptube-transcript-languages', State.transcriptLanguages);
  });
  $('#model-select').addEventListener('change', (e) => {
    State.selectedModel = e.target.value;
    localStorage.setItem('cliptube-model-name', State.selectedModel);
    $('#chat-model-name').textContent = State.selectedModel;
    $('#btn-analyze').disabled = !State.transcript || !State.selectedModel;
  });
  $('#btn-refresh-models').addEventListener('click', loadModels);
  $$('.style-btn').forEach((btn) => {
    btn.addEventListener('click', () => {
      State.outputStyle = Number(btn.dataset.style);
      localStorage.setItem('cliptube-output-style', String(State.outputStyle));
      syncStyleButtons();
      $('#chat-style').value = String(State.outputStyle);
      if (State.summary) {
        State.shareText = buildShareText(State.summary);
        $('#result-share').textContent = State.shareText;
      }
    });
  });
  $('#chat-style').addEventListener('change', (e) => {
    State.outputStyle = Number(e.target.value);
    localStorage.setItem('cliptube-output-style', String(State.outputStyle));
    syncStyleButtons();
  });
  $('#btn-fetch').addEventListener('click', fetchTranscript);
  $('#youtube-url').addEventListener('keydown', (e) => {
    if (e.key === 'Enter') fetchTranscript();
  });
  $('#btn-analyze').addEventListener('click', analyzeVideo);
  $('#btn-toggle-transcript').addEventListener('click', () => {
    const el = $('#result-transcript');
    const btn = $('#btn-toggle-transcript');
    el.classList.toggle('collapsed');
    el.classList.toggle('open');
    btn.textContent = el.classList.contains('collapsed') ? t('btnShow') : t('btnHide');
  });
  $('#btn-copy-share').addEventListener('click', copyShareText);
  $('#btn-chat-send').addEventListener('click', sendChat);
  $('#btn-chat-improve').addEventListener('click', improveFinalText);
  $('#chat-input').addEventListener('keydown', (e) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      sendChat();
    }
  });
}

document.addEventListener('DOMContentLoaded', () => {
  State.lang = normalizeUiLanguage(State.lang);
  renderLanguageOptions();
  loadSettings();
  bindEvents();
  setupInstallPrompt();
  applyLanguage(State.lang);
  loadModels();
  renderChatHistory();
  $('#chat-style').value = String(State.outputStyle);
  syncStyleButtons();
  updateRuntimeBanner();
  registerServiceWorker();
});
