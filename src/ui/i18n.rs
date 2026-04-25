#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UiLanguage {
    Es,
    En,
    Pt,
    Fr,
    De,
    Ja,
    ZhHans,
    Ru,
    Ar,
    Hi,
}

impl UiLanguage {
    pub fn from_code(code: &str) -> Option<Self> {
        let normalized = code.trim().to_lowercase();
        let base = normalized.split(['-', '_']).next().unwrap_or_default();

        match base {
            "es" => Some(Self::Es),
            "en" => Some(Self::En),
            "pt" => Some(Self::Pt),
            "fr" => Some(Self::Fr),
            "de" => Some(Self::De),
            "ja" => Some(Self::Ja),
            "zh" => Some(Self::ZhHans),
            "ru" => Some(Self::Ru),
            "ar" => Some(Self::Ar),
            "hi" => Some(Self::Hi),
            _ => None,
        }
    }

    pub fn detect_system() -> Self {
        if let Ok(forced) = std::env::var("CLIPTUBE_UI_LANGUAGE")
            && let Some(language) = Self::from_code(&forced)
        {
            return language;
        }

        let locale = std::env::var("LC_ALL")
            .ok()
            .or_else(|| std::env::var("LANG").ok())
            .unwrap_or_default()
            .to_lowercase();

        Self::from_code(&locale).unwrap_or(Self::Es)
    }

    pub fn code(self) -> &'static str {
        match self {
            Self::Es => "es",
            Self::En => "en",
            Self::Pt => "pt",
            Self::Fr => "fr",
            Self::De => "de",
            Self::Ja => "ja",
            Self::ZhHans => "zh-Hans",
            Self::Ru => "ru",
            Self::Ar => "ar",
            Self::Hi => "hi",
        }
    }

    pub fn prefers_transcript_languages(self) -> &'static str {
        match self {
            Self::Es => "es,en",
            Self::En => "en,es",
            Self::Pt => "pt,es,en",
            Self::Fr => "fr,en,es",
            Self::De => "de,en,es",
            Self::Ja => "ja,en",
            Self::ZhHans => "zh,en",
            Self::Ru => "ru,en",
            Self::Ar => "ar,en",
            Self::Hi => "hi,en",
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::Es => "Español",
            Self::En => "English",
            Self::Pt => "Português",
            Self::Fr => "Français",
            Self::De => "Deutsch",
            Self::Ja => "Japanese",
            Self::ZhHans => "Chinese (Simplified)",
            Self::Ru => "Russian",
            Self::Ar => "Arabic",
            Self::Hi => "Hindi",
        }
    }

    pub fn country_code(self) -> &'static str {
        match self {
            Self::Es => "ES",
            Self::En => "US",
            Self::Pt => "BR",
            Self::Fr => "FR",
            Self::De => "DE",
            Self::Ja => "JP",
            Self::ZhHans => "CN",
            Self::Ru => "RU",
            Self::Ar => "SA",
            Self::Hi => "IN",
        }
    }

    pub fn all() -> &'static [Self] {
        &[
            Self::En,
            Self::Es,
            Self::Pt,
            Self::Fr,
            Self::De,
            Self::Ja,
            Self::ZhHans,
            Self::Ru,
            Self::Ar,
            Self::Hi,
        ]
    }

    pub fn is_rtl(self) -> bool {
        matches!(self, Self::Ar)
    }

    fn extended_text(self, key: &'static str) -> Option<&'static str> {
        match self {
            Self::Fr => match key {
                "app_desc" => Some(
                    "Application native en Rust. Récupère la transcription, génère un résumé avec Ollama et produit un texte prêt à partager.",
                ),
                "status_init" => Some("Collez une URL ou un ID YouTube puis générez un résumé."),
                "video" => Some("Vidéo"),
                "languages" => Some("Langues"),
                "model" => Some("Modèle"),
                "reload_models" => Some("Recharger les modèles depuis Ollama"),
                "custom_model" => Some("modèle personnalisé"),
                "format" => Some("Format"),
                "run" => Some("Récupérer la transcription et résumer"),
                "processing" => Some("Traitement en cours..."),
                "copy_final" => Some("📋 Copier le texte final"),
                "video_details" => Some("Détails de la vidéo"),
                "video_config_desc" => Some(
                    "Configurez la vidéo, le modèle et le type de sortie avant de lancer l’analyse.",
                ),
                "video_meta_desc" => {
                    Some("Métadonnées utiles de la vidéo et de la transcription détectée.")
                }
                "share_text" => Some("Texte prêt à coller"),
                "full_transcript" => Some("Transcription complète"),
                "chat_title" => Some("Chat IA"),
                "chat_desc" => {
                    Some("Posez une question sur la vidéo ou améliorez le texte final déjà généré.")
                }
                "chat_initial" => Some(
                    "Posez-moi une question sur la vidéo ou utilisez le bouton pour améliorer le texte prêt à coller.",
                ),
                "you" => Some("Vous"),
                "assistant" => Some("IA"),
                "question" => Some("Question ou instruction"),
                "question_hint" => Some(
                    "Ex. : améliore l’accroche initiale, rends-le plus exécutif, réponds à une question sur la vidéo...",
                ),
                "ask_ai" => Some("Demander à l’IA"),
                "improve_final" => Some("Améliorer le texte final"),
                "status_need_url" => Some("Vous devez coller une URL ou un ID YouTube."),
                "status_fetching" => {
                    Some("Récupération de la transcription et génération du résumé...")
                }
                "status_context_ready" => Some(
                    "J’ai déjà le contexte de la vidéo. Vous pouvez me poser des questions ou me demander d’améliorer le texte final.",
                ),
                "status_improved" => Some("Texte final amélioré avec l’IA."),
                "status_chat_ready" => Some("Réponse du chat prête."),
                "models_loaded" => Some("Modèles chargés depuis Ollama"),
                "status_need_transcript" => {
                    Some("Vous devez d’abord récupérer la transcription de la vidéo.")
                }
                "status_need_question" => Some("Écrivez une question pour l’IA."),
                "status_consulting_ai" => Some("Consultation de l’IA..."),
                "status_loading_models" => {
                    Some("Chargement des modèles disponibles depuis Ollama...")
                }
                "clipboard_ok" => Some("Texte final copié dans le presse-papiers."),
                "models_noun" => Some("modèles"),
                "summary" => Some("Résumé"),
                "key_points" => Some("Points importants"),
                "language" => Some("Langue"),
                "type" => Some("Type"),
                "output" => Some("Sortie"),
                "transcript" => Some("Transcription"),
                "characters" => Some("caractères"),
                "auto_subtitles" => Some("sous-titres automatiques"),
                "manual_subtitles" => Some("sous-titres manuels"),
                "no_key_points" => Some("Aucun point structuré n’a pu être détecté."),
                "chat_format" => Some("Chat prêt à coller"),
                "executive_format" => Some("Résumé exécutif"),
                "bullets_format" => Some("Points directs"),
                "message" => Some("Message"),
                _ => None,
            },
            Self::De => match key {
                "app_desc" => Some(
                    "Native Rust-App. Holt das Transkript, erstellt mit Ollama eine Zusammenfassung und baut sofort teilbaren Text.",
                ),
                "status_init" => Some(
                    "Fügen Sie eine YouTube-URL oder -ID ein und erzeugen Sie eine Zusammenfassung.",
                ),
                "video" => Some("Video"),
                "languages" => Some("Sprachen"),
                "model" => Some("Modell"),
                "reload_models" => Some("Modelle aus Ollama neu laden"),
                "custom_model" => Some("benutzerdefiniertes Modell"),
                "format" => Some("Format"),
                "run" => Some("Transkript holen und zusammenfassen"),
                "processing" => Some("Wird verarbeitet..."),
                "copy_final" => Some("📋 Finalen Text kopieren"),
                "video_details" => Some("Videodetails"),
                "video_config_desc" => Some(
                    "Konfigurieren Sie Video, Modell und Ausgabetyp, bevor Sie die Analyse starten.",
                ),
                "video_meta_desc" => {
                    Some("Nützliche Metadaten des Videos und des erkannten Transkripts.")
                }
                "share_text" => Some("Text zum Einfügen"),
                "full_transcript" => Some("Vollständiges Transkript"),
                "chat_title" => Some("KI-Chat"),
                "chat_desc" => Some(
                    "Fragen Sie nach dem Video oder verbessern Sie den bereits erzeugten Endtext.",
                ),
                "chat_initial" => Some(
                    "Fragen Sie mich zum Video oder nutzen Sie die Schaltfläche, um den Text zum Einfügen zu verbessern.",
                ),
                "you" => Some("Sie"),
                "assistant" => Some("KI"),
                "question" => Some("Frage oder Anweisung"),
                "question_hint" => Some(
                    "Z. B.: verbessere den Einstieg, mach ihn executive-tauglicher, beantworte eine Frage zum Video...",
                ),
                "ask_ai" => Some("KI fragen"),
                "improve_final" => Some("Finalen Text verbessern"),
                "status_need_url" => Some("Sie müssen eine YouTube-URL oder -ID einfügen."),
                "status_fetching" => {
                    Some("Transkript wird geladen und Zusammenfassung erstellt...")
                }
                "status_context_ready" => Some(
                    "Ich habe bereits den Kontext des Videos. Sie können mir Fragen stellen oder den finalen Text verbessern lassen.",
                ),
                "status_improved" => Some("Finaler Text mit KI verbessert."),
                "status_chat_ready" => Some("Chat-Antwort bereit."),
                "models_loaded" => Some("Aus Ollama geladene Modelle"),
                "status_need_transcript" => {
                    Some("Sie müssen zuerst das Transkript des Videos abrufen.")
                }
                "status_need_question" => Some("Schreiben Sie eine Frage für die KI."),
                "status_consulting_ai" => Some("KI wird konsultiert..."),
                "status_loading_models" => Some("Verfügbare Modelle von Ollama werden geladen..."),
                "clipboard_ok" => Some("Finaler Text in die Zwischenablage kopiert."),
                "models_noun" => Some("Modelle"),
                "summary" => Some("Zusammenfassung"),
                "key_points" => Some("Wichtige Punkte"),
                "language" => Some("Sprache"),
                "type" => Some("Typ"),
                "output" => Some("Ausgabe"),
                "transcript" => Some("Transkript"),
                "characters" => Some("Zeichen"),
                "auto_subtitles" => Some("automatische Untertitel"),
                "manual_subtitles" => Some("manuelle Untertitel"),
                "no_key_points" => {
                    Some("Es konnten keine strukturierten Schlüsselpunkte erkannt werden.")
                }
                "chat_format" => Some("Chat zum Einfügen"),
                "executive_format" => Some("Management-Zusammenfassung"),
                "bullets_format" => Some("Direkte Stichpunkte"),
                "message" => Some("Nachricht"),
                _ => None,
            },
            Self::Ja => match key {
                "app_desc" => Some(
                    "Rust 製のネイティブアプリ。文字起こしを取得し、Ollama で要約を生成し、共有しやすいテキストを作成します。",
                ),
                "status_init" => Some("YouTube の URL または ID を貼り付けて要約を生成します。"),
                "video" => Some("動画"),
                "languages" => Some("言語"),
                "model" => Some("モデル"),
                "reload_models" => Some("Ollama からモデルを再読み込み"),
                "custom_model" => Some("カスタムモデル"),
                "format" => Some("形式"),
                "run" => Some("文字起こしを取得して要約"),
                "processing" => Some("処理中..."),
                "copy_final" => Some("📋 最終テキストをコピー"),
                "video_details" => Some("動画の詳細"),
                "video_config_desc" => {
                    Some("分析を始める前に、動画、モデル、出力形式を設定してください。")
                }
                "video_meta_desc" => Some("動画と検出された文字起こしの便利なメタデータです。"),
                "share_text" => Some("貼り付け用テキスト"),
                "full_transcript" => Some("全文文字起こし"),
                "chat_title" => Some("AI チャット"),
                "chat_desc" => {
                    Some("動画について質問したり、生成済みの最終テキストを改善できます。")
                }
                "chat_initial" => Some(
                    "動画について質問するか、ボタンを使って貼り付け用テキストを改善してください。",
                ),
                "you" => Some("あなた"),
                "assistant" => Some("AI"),
                "question" => Some("質問または指示"),
                "question_hint" => Some(
                    "例: 冒頭のフックを改善する、もっとエグゼクティブ向けにする、動画についての質問に答える...",
                ),
                "ask_ai" => Some("AI に質問"),
                "improve_final" => Some("最終テキストを改善"),
                "status_need_url" => Some("YouTube の URL または ID を貼り付けてください。"),
                "status_fetching" => Some("文字起こしを取得して要約を生成しています..."),
                "status_context_ready" => Some(
                    "動画のコンテキストを取得しました。質問したり、最終テキストの改善を依頼できます。",
                ),
                "status_improved" => Some("AI により最終テキストを改善しました。"),
                "status_chat_ready" => Some("チャットの応答が準備できました。"),
                "models_loaded" => Some("Ollama から読み込んだモデル"),
                "status_need_transcript" => Some("まず動画の文字起こしを取得してください。"),
                "status_need_question" => Some("AI への質問を入力してください。"),
                "status_consulting_ai" => Some("AI に問い合わせ中..."),
                "status_loading_models" => Some("Ollama から利用可能なモデルを読み込んでいます..."),
                "clipboard_ok" => Some("最終テキストをクリップボードにコピーしました。"),
                "models_noun" => Some("モデル"),
                "summary" => Some("要約"),
                "key_points" => Some("重要ポイント"),
                "language" => Some("言語"),
                "type" => Some("種類"),
                "output" => Some("出力"),
                "transcript" => Some("文字起こし"),
                "characters" => Some("文字"),
                "auto_subtitles" => Some("自動字幕"),
                "manual_subtitles" => Some("手動字幕"),
                "no_key_points" => Some("構造化された重要ポイントを検出できませんでした。"),
                "chat_format" => Some("貼り付け用チャット"),
                "executive_format" => Some("エグゼクティブ要約"),
                "bullets_format" => Some("箇条書き"),
                "message" => Some("メッセージ"),
                _ => None,
            },
            Self::ZhHans => match key {
                "app_desc" => Some(
                    "原生 Rust 应用。获取转录内容，使用 Ollama 生成摘要，并整理出可直接分享的文本。",
                ),
                "status_init" => Some("粘贴 YouTube 链接或 ID 并生成摘要。"),
                "video" => Some("视频"),
                "languages" => Some("语言"),
                "model" => Some("模型"),
                "reload_models" => Some("从 Ollama 重新加载模型"),
                "custom_model" => Some("自定义模型"),
                "format" => Some("格式"),
                "run" => Some("获取转录并总结"),
                "processing" => Some("处理中..."),
                "copy_final" => Some("📋 复制最终文本"),
                "video_details" => Some("视频详情"),
                "video_config_desc" => Some("开始分析前，请先配置视频、模型和输出类型。"),
                "video_meta_desc" => Some("视频和检测到的转录文本的有用元数据。"),
                "share_text" => Some("可直接粘贴的文本"),
                "full_transcript" => Some("完整转录"),
                "chat_title" => Some("AI 聊天"),
                "chat_desc" => Some("你可以就视频提问，或改进已生成的最终文本。"),
                "chat_initial" => Some("向我提问视频内容，或使用按钮改进可直接粘贴的文本。"),
                "you" => Some("你"),
                "assistant" => Some("AI"),
                "question" => Some("问题或指令"),
                "question_hint" => {
                    Some("例如：优化开头钩子、让内容更适合汇报、回答关于视频的问题...")
                }
                "ask_ai" => Some("询问 AI"),
                "improve_final" => Some("改进最终文本"),
                "status_need_url" => Some("你必须粘贴 YouTube 链接或 ID。"),
                "status_fetching" => Some("正在获取转录并生成摘要..."),
                "status_context_ready" => {
                    Some("我已经有视频上下文了。你可以继续提问，或让我改进最终文本。")
                }
                "status_improved" => Some("最终文本已通过 AI 改进。"),
                "status_chat_ready" => Some("聊天回复已准备好。"),
                "models_loaded" => Some("已从 Ollama 加载的模型"),
                "status_need_transcript" => Some("请先获取视频的转录内容。"),
                "status_need_question" => Some("请为 AI 输入一个问题。"),
                "status_consulting_ai" => Some("正在咨询 AI..."),
                "status_loading_models" => Some("正在从 Ollama 加载可用模型..."),
                "clipboard_ok" => Some("最终文本已复制到剪贴板。"),
                "models_noun" => Some("个模型"),
                "summary" => Some("摘要"),
                "key_points" => Some("重点"),
                "language" => Some("语言"),
                "type" => Some("类型"),
                "output" => Some("输出"),
                "transcript" => Some("转录"),
                "characters" => Some("字符"),
                "auto_subtitles" => Some("自动字幕"),
                "manual_subtitles" => Some("手动字幕"),
                "no_key_points" => Some("未能检测到结构化重点。"),
                "chat_format" => Some("可直接粘贴的聊天文本"),
                "executive_format" => Some("执行摘要"),
                "bullets_format" => Some("直接要点"),
                "message" => Some("消息"),
                _ => None,
            },
            Self::Ru => match key {
                "app_desc" => Some(
                    "Нативное приложение на Rust. Получает транскрипт, генерирует сводку через Ollama и собирает готовый к отправке текст.",
                ),
                "status_init" => Some("Вставьте URL или ID YouTube и создайте сводку."),
                "video" => Some("Видео"),
                "languages" => Some("Языки"),
                "model" => Some("Модель"),
                "reload_models" => Some("Перезагрузить модели из Ollama"),
                "custom_model" => Some("пользовательская модель"),
                "format" => Some("Формат"),
                "run" => Some("Получить транскрипт и сводку"),
                "processing" => Some("Обработка..."),
                "copy_final" => Some("📋 Скопировать итоговый текст"),
                "video_details" => Some("Детали видео"),
                "video_config_desc" => {
                    Some("Настройте видео, модель и тип вывода перед запуском анализа.")
                }
                "video_meta_desc" => Some("Полезные метаданные видео и обнаруженного транскрипта."),
                "share_text" => Some("Текст для вставки"),
                "full_transcript" => Some("Полный транскрипт"),
                "chat_title" => Some("AI-чат"),
                "chat_desc" => {
                    Some("Спросите о видео или улучшите уже сгенерированный итоговый текст.")
                }
                "chat_initial" => Some(
                    "Спросите меня о видео или используйте кнопку, чтобы улучшить текст для вставки.",
                ),
                "you" => Some("Вы"),
                "assistant" => Some("ИИ"),
                "question" => Some("Вопрос или инструкция"),
                "question_hint" => Some(
                    "Напр.: улучши вступление, сделай текст более executive, ответь на вопрос по видео...",
                ),
                "ask_ai" => Some("Спросить ИИ"),
                "improve_final" => Some("Улучшить итоговый текст"),
                "status_need_url" => Some("Нужно вставить URL или ID YouTube."),
                "status_fetching" => Some("Получаем транскрипт и создаём сводку..."),
                "status_context_ready" => Some(
                    "У меня уже есть контекст видео. Вы можете задать вопрос или попросить улучшить итоговый текст.",
                ),
                "status_improved" => Some("Итоговый текст улучшен с помощью ИИ."),
                "status_chat_ready" => Some("Ответ чата готов."),
                "models_loaded" => Some("Модели, загруженные из Ollama"),
                "status_need_transcript" => Some("Сначала нужно получить транскрипт видео."),
                "status_need_question" => Some("Введите вопрос для ИИ."),
                "status_consulting_ai" => Some("Обращение к ИИ..."),
                "status_loading_models" => Some("Загрузка доступных моделей из Ollama..."),
                "clipboard_ok" => Some("Итоговый текст скопирован в буфер обмена."),
                "models_noun" => Some("моделей"),
                "summary" => Some("Сводка"),
                "key_points" => Some("Ключевые пункты"),
                "language" => Some("Язык"),
                "type" => Some("Тип"),
                "output" => Some("Вывод"),
                "transcript" => Some("Транскрипт"),
                "characters" => Some("символов"),
                "auto_subtitles" => Some("автоматические субтитры"),
                "manual_subtitles" => Some("ручные субтитры"),
                "no_key_points" => Some("Не удалось определить структурированные ключевые пункты."),
                "chat_format" => Some("Текст для чата"),
                "executive_format" => Some("Краткое резюме"),
                "bullets_format" => Some("Короткие пункты"),
                "message" => Some("Сообщение"),
                _ => None,
            },
            Self::Ar => match key {
                "app_desc" => Some(
                    "تطبيق Rust أصلي. يجلب النص المفرغ، ويولّد ملخصًا عبر Ollama، ويجهز نصًا قابلًا للمشاركة.",
                ),
                "status_init" => Some("ألصق رابط YouTube أو المعرّف ثم أنشئ ملخصًا."),
                "video" => Some("الفيديو"),
                "languages" => Some("اللغات"),
                "model" => Some("النموذج"),
                "reload_models" => Some("إعادة تحميل النماذج من Ollama"),
                "custom_model" => Some("نموذج مخصص"),
                "format" => Some("التنسيق"),
                "run" => Some("جلب النص المفرغ وتلخيصه"),
                "processing" => Some("جارٍ المعالجة..."),
                "copy_final" => Some("📋 نسخ النص النهائي"),
                "video_details" => Some("تفاصيل الفيديو"),
                "video_config_desc" => Some("اضبط الفيديو والنموذج ونوع المخرجات قبل بدء التحليل."),
                "video_meta_desc" => Some("بيانات مفيدة من الفيديو والنص المفرغ المكتشف."),
                "share_text" => Some("نص جاهز للصق"),
                "full_transcript" => Some("النص المفرغ الكامل"),
                "chat_title" => Some("دردشة الذكاء الاصطناعي"),
                "chat_desc" => Some("اسأل عن الفيديو أو حسّن النص النهائي الذي تم إنشاؤه."),
                "chat_initial" => Some("اسألني عن الفيديو أو استخدم الزر لتحسين النص الجاهز للصق."),
                "you" => Some("أنت"),
                "assistant" => Some("الذكاء الاصطناعي"),
                "question" => Some("سؤال أو تعليمات"),
                "question_hint" => {
                    Some("مثال: حسّن المقدمة، اجعلها أكثر تنفيذية، أجب عن سؤال يتعلق بالفيديو...")
                }
                "ask_ai" => Some("اسأل الذكاء الاصطناعي"),
                "improve_final" => Some("تحسين النص النهائي"),
                "status_need_url" => Some("يجب لصق رابط YouTube أو المعرّف."),
                "status_fetching" => Some("جارٍ جلب النص المفرغ وإنشاء الملخص..."),
                "status_context_ready" => {
                    Some("أصبح لديّ سياق الفيديو. يمكنك أن تسألني أو تطلب تحسين النص النهائي.")
                }
                "status_improved" => Some("تم تحسين النص النهائي بالذكاء الاصطناعي."),
                "status_chat_ready" => Some("رد الدردشة جاهز."),
                "models_loaded" => Some("النماذج المحمّلة من Ollama"),
                "status_need_transcript" => Some("يجب أولًا جلب النص المفرغ للفيديو."),
                "status_need_question" => Some("اكتب سؤالًا للذكاء الاصطناعي."),
                "status_consulting_ai" => Some("جارٍ استشارة الذكاء الاصطناعي..."),
                "status_loading_models" => Some("جارٍ تحميل النماذج المتاحة من Ollama..."),
                "clipboard_ok" => Some("تم نسخ النص النهائي إلى الحافظة."),
                "models_noun" => Some("نماذج"),
                "summary" => Some("الملخص"),
                "key_points" => Some("النقاط المهمة"),
                "language" => Some("اللغة"),
                "type" => Some("النوع"),
                "output" => Some("المخرجات"),
                "transcript" => Some("النص المفرغ"),
                "characters" => Some("أحرف"),
                "auto_subtitles" => Some("ترجمة تلقائية"),
                "manual_subtitles" => Some("ترجمة يدوية"),
                "no_key_points" => Some("تعذر اكتشاف نقاط مهمة منظمة."),
                "chat_format" => Some("نص جاهز للدردشة"),
                "executive_format" => Some("ملخص تنفيذي"),
                "bullets_format" => Some("نقاط مباشرة"),
                "message" => Some("رسالة"),
                _ => None,
            },
            Self::Hi => match key {
                "app_desc" => Some(
                    "Rust में बना नेटिव ऐप। ट्रांसक्रिप्ट लाता है, Ollama से सारांश बनाता है और साझा करने लायक टेक्स्ट तैयार करता है।",
                ),
                "status_init" => Some("YouTube URL या ID पेस्ट करें और सारांश बनाएं।"),
                "video" => Some("वीडियो"),
                "languages" => Some("भाषाएँ"),
                "model" => Some("मॉडल"),
                "reload_models" => Some("Ollama से मॉडल फिर से लोड करें"),
                "custom_model" => Some("कस्टम मॉडल"),
                "format" => Some("फ़ॉर्मेट"),
                "run" => Some("ट्रांसक्रिप्ट लाएँ और सारांश बनाएँ"),
                "processing" => Some("प्रोसेस हो रहा है..."),
                "copy_final" => Some("📋 अंतिम टेक्स्ट कॉपी करें"),
                "video_details" => Some("वीडियो विवरण"),
                "video_config_desc" => {
                    Some("विश्लेषण शुरू करने से पहले वीडियो, मॉडल और आउटपुट प्रकार सेट करें।")
                }
                "video_meta_desc" => Some("वीडियो और पहचाने गए ट्रांसक्रिप्ट का उपयोगी मेटाडेटा।"),
                "share_text" => Some("पेस्ट करने के लिए टेक्स्ट"),
                "full_transcript" => Some("पूरा ट्रांसक्रिप्ट"),
                "chat_title" => Some("AI चैट"),
                "chat_desc" => Some("वीडियो के बारे में पूछें या बनाए गए अंतिम टेक्स्ट को बेहतर करें।"),
                "chat_initial" => {
                    Some("मुझसे वीडियो के बारे में पूछें या बटन का उपयोग करके पेस्ट-रेडी टेक्स्ट बेहतर करें।")
                }
                "you" => Some("आप"),
                "assistant" => Some("AI"),
                "question" => Some("प्रश्न या निर्देश"),
                "question_hint" => Some(
                    "उदा.: शुरुआती हुक बेहतर करो, इसे अधिक executive बनाओ, वीडियो के बारे में सवाल का जवाब दो...",
                ),
                "ask_ai" => Some("AI से पूछें"),
                "improve_final" => Some("अंतिम टेक्स्ट सुधारें"),
                "status_need_url" => Some("आपको YouTube URL या ID पेस्ट करनी होगी।"),
                "status_fetching" => Some("ट्रांसक्रिप्ट लाया जा रहा है और सारांश बनाया जा रहा है..."),
                "status_context_ready" => Some(
                    "मेरे पास वीडियो का संदर्भ है। आप मुझसे सवाल पूछ सकते हैं या अंतिम टेक्स्ट सुधारने को कह सकते हैं।",
                ),
                "status_improved" => Some("AI ने अंतिम टेक्स्ट सुधार दिया है।"),
                "status_chat_ready" => Some("चैट उत्तर तैयार है।"),
                "models_loaded" => Some("Ollama से लोड किए गए मॉडल"),
                "status_need_transcript" => Some("पहले वीडियो का ट्रांसक्रिप्ट लाना होगा।"),
                "status_need_question" => Some("AI के लिए एक प्रश्न लिखें।"),
                "status_consulting_ai" => Some("AI से परामर्श किया जा रहा है..."),
                "status_loading_models" => Some("Ollama से उपलब्ध मॉडल लोड किए जा रहे हैं..."),
                "clipboard_ok" => Some("अंतिम टेक्स्ट क्लिपबोर्ड में कॉपी हो गया।"),
                "models_noun" => Some("मॉडल"),
                "summary" => Some("सारांश"),
                "key_points" => Some("महत्वपूर्ण बिंदु"),
                "language" => Some("भाषा"),
                "type" => Some("प्रकार"),
                "output" => Some("आउटपुट"),
                "transcript" => Some("ट्रांसक्रिप्ट"),
                "characters" => Some("अक्षर"),
                "auto_subtitles" => Some("स्वचालित सबटाइटल"),
                "manual_subtitles" => Some("मैनुअल सबटाइटल"),
                "no_key_points" => Some("कोई संरचित मुख्य बिंदु नहीं मिले।"),
                "chat_format" => Some("पेस्ट-रेडी चैट"),
                "executive_format" => Some("कार्यकारी सारांश"),
                "bullets_format" => Some("सीधे बुलेट पॉइंट्स"),
                "message" => Some("संदेश"),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn text(self, key: &'static str) -> &'static str {
        if let Some(value) = self.extended_text(key) {
            return value;
        }
        match (self, key) {
            (_, "app_title") => "ClipTube AI",
            (
                Self::Es,
                "ollama_connection"
                | "connection_desc"
                | "ollama_host"
                | "ollama_port"
                | "endpoint_override"
                | "endpoint_override_hint"
                | "effective_url"
                | "status_invalid_connection",
            ) => match key {
                "ollama_connection" => "Conexión Ollama",
                "connection_desc" => {
                    "Configura host, puerto o un endpoint completo para usarlo tanto al cargar modelos como al resumir."
                }
                "ollama_host" => "Host",
                "ollama_port" => "Puerto",
                "endpoint_override" => "Endpoint completo (opcional)",
                "endpoint_override_hint" => "Si lo defines, reemplaza host + puerto",
                "effective_url" => "URL efectiva",
                "status_invalid_connection" => {
                    "Configura un host y puerto válidos, o un endpoint completo correcto."
                }
                _ => key,
            },
            (
                Self::En,
                "ollama_connection"
                | "connection_desc"
                | "ollama_host"
                | "ollama_port"
                | "endpoint_override"
                | "endpoint_override_hint"
                | "effective_url"
                | "status_invalid_connection",
            ) => match key {
                "ollama_connection" => "Ollama connection",
                "connection_desc" => {
                    "Configure host, port, or a full endpoint override to use it for model loading and AI generation."
                }
                "ollama_host" => "Host",
                "ollama_port" => "Port",
                "endpoint_override" => "Full endpoint override (optional)",
                "endpoint_override_hint" => "If set, it overrides host + port",
                "effective_url" => "Effective URL",
                "status_invalid_connection" => {
                    "Set a valid host and port, or a valid full endpoint override."
                }
                _ => key,
            },
            (
                Self::Pt,
                "ollama_connection"
                | "connection_desc"
                | "ollama_host"
                | "ollama_port"
                | "endpoint_override"
                | "endpoint_override_hint"
                | "effective_url"
                | "status_invalid_connection",
            ) => match key {
                "ollama_connection" => "Conexão Ollama",
                "connection_desc" => {
                    "Configure host, porta ou um endpoint completo para usar no carregamento de modelos e na geração com IA."
                }
                "ollama_host" => "Host",
                "ollama_port" => "Porta",
                "endpoint_override" => "Endpoint completo (opcional)",
                "endpoint_override_hint" => "Se definido, substitui host + porta",
                "effective_url" => "URL efetiva",
                "status_invalid_connection" => {
                    "Defina um host e porta válidos, ou um endpoint completo válido."
                }
                _ => key,
            },
            (
                Self::Es,
                "app_desc"
                | "status_init"
                | "video"
                | "languages"
                | "model"
                | "reload_models"
                | "custom_model"
                | "format"
                | "run"
                | "processing"
                | "copy_final"
                | "video_details"
                | "video_config_desc"
                | "video_meta_desc"
                | "share_text"
                | "full_transcript"
                | "chat_title"
                | "chat_desc"
                | "chat_initial"
                | "you"
                | "assistant"
                | "question"
                | "question_hint"
                | "ask_ai"
                | "improve_final"
                | "status_need_url"
                | "status_fetching"
                | "status_context_ready"
                | "status_improved"
                | "status_chat_ready"
                | "status_loaded_from_history"
                | "models_loaded"
                | "status_need_transcript"
                | "status_need_question"
                | "status_consulting_ai"
                | "status_loading_models"
                | "clipboard_ok"
                | "models_noun"
                | "summary"
                | "key_points"
                | "language"
                | "type"
                | "output"
                | "transcript"
                | "characters"
                | "auto_subtitles"
                | "manual_subtitles"
                | "no_key_points"
                | "chat_format"
                | "executive_format"
                | "bullets_format"
                | "message",
            ) => match key {
                "app_desc" => {
                    "App nativa en Rust. Obtiene transcript, genera resumen con Ollama y arma texto listo para compartir."
                }
                "status_init" => "Pega una URL o ID de YouTube y genera un resumen.",
                "video" => "Video",
                "languages" => "Idiomas",
                "model" => "Modelo",
                "reload_models" => "Recargar modelos desde Ollama",
                "custom_model" => "modelo personalizado",
                "format" => "Formato",
                "run" => "Obtener transcript y resumir",
                "processing" => "Procesando...",
                "copy_final" => "📋 Copiar texto final",
                "video_details" => "Detalle del video",
                "video_config_desc" => {
                    "Configura el video, modelo y tipo de salida antes de lanzar el análisis."
                }
                "video_meta_desc" => "Metadatos útiles del video y del transcript detectado.",
                "share_text" => "Texto listo para pegar",
                "full_transcript" => "Transcripción completa",
                "chat_title" => "Chat IA",
                "chat_desc" => "Pregúntale al video o mejora el texto final ya creado.",
                "chat_initial" => {
                    "Pregúntame sobre el video o usa el botón para mejorar el texto listo para pegar."
                }
                "you" => "Tú",
                "assistant" => "IA",
                "question" => "Pregunta o instrucción",
                "question_hint" => {
                    "Ej: mejora el gancho inicial, hazlo más ejecutivo, responde una duda del video..."
                }
                "ask_ai" => "Preguntar a la IA",
                "improve_final" => "Mejorar texto final",
                "status_need_url" => "Debes pegar una URL o ID de YouTube.",
                "status_fetching" => "Buscando transcript y generando resumen...",
                "status_context_ready" => {
                    "Ya tengo el contexto del video. Puedes preguntarme cosas o pedirme mejorar el texto final."
                }
                "status_improved" => "Texto final mejorado con IA.",
                "status_chat_ready" => "Respuesta del chat lista.",
                "status_loaded_from_history" => "Video cargado desde el historial.",
                "models_loaded" => "Modelos cargados desde Ollama",
                "status_need_transcript" => "Primero debes obtener el transcript del video.",
                "status_need_question" => "Escribe una pregunta para la IA.",
                "status_consulting_ai" => "Consultando a la IA...",
                "status_loading_models" => "Consultando modelos disponibles en Ollama...",
                "clipboard_ok" => "Texto final copiado al portapapeles.",
                "models_noun" => "modelos",
                "summary" => "Resumen",
                "key_points" => "Puntos Importantes",
                "language" => "Idioma",
                "type" => "Tipo",
                "output" => "Salida",
                "transcript" => "Transcripción",
                "characters" => "caracteres",
                "auto_subtitles" => "subtítulo automático",
                "manual_subtitles" => "subtítulo manual",
                "no_key_points" => "No se pudieron detectar puntos importantes estructurados.",
                "chat_format" => "Chat listo para pegar",
                "executive_format" => "Resumen ejecutivo",
                "bullets_format" => "Bullets directos",
                "message" => "Mensaje",
                "history" => "Historial",
                "import_history" => "Importar historial de YouTube",
                "load" => "Cargar",
                "delete" => "Eliminar",
                "no_history" => "No hay videos guardados",
                "history_imported" => "Historial importado",
                _ => key,
            },
            (
                Self::En,
                "app_desc"
                | "status_init"
                | "video"
                | "languages"
                | "model"
                | "reload_models"
                | "custom_model"
                | "format"
                | "run"
                | "processing"
                | "copy_final"
                | "video_details"
                | "video_config_desc"
                | "video_meta_desc"
                | "share_text"
                | "full_transcript"
                | "chat_title"
                | "chat_desc"
                | "chat_initial"
                | "you"
                | "assistant"
                | "question"
                | "question_hint"
                | "ask_ai"
                | "improve_final"
                | "status_need_url"
                | "status_fetching"
                | "status_context_ready"
                | "status_improved"
                | "status_chat_ready"
                | "status_loaded_from_history"
                | "models_loaded"
                | "status_need_transcript"
                | "status_need_question"
                | "status_consulting_ai"
                | "status_loading_models"
                | "clipboard_ok"
                | "models_noun"
                | "summary"
                | "key_points"
                | "language"
                | "type"
                | "output"
                | "transcript"
                | "characters"
                | "auto_subtitles"
                | "manual_subtitles"
                | "no_key_points"
                | "chat_format"
                | "executive_format"
                | "bullets_format"
                | "message",
            ) => match key {
                "app_desc" => {
                    "Native Rust app. Fetches transcript, generates a summary with Ollama, and builds share-ready text."
                }
                "status_init" => "Paste a YouTube URL or ID and generate a summary.",
                "video" => "Video",
                "languages" => "Languages",
                "model" => "Model",
                "reload_models" => "Reload models from Ollama",
                "custom_model" => "custom model",
                "format" => "Format",
                "run" => "Fetch transcript and summarize",
                "processing" => "Processing...",
                "copy_final" => "📋 Copy final text",
                "video_details" => "Video details",
                "video_config_desc" => {
                    "Configure the video, model, and output type before starting the analysis."
                }
                "video_meta_desc" => "Useful metadata from the video and the detected transcript.",
                "share_text" => "Ready-to-paste text",
                "full_transcript" => "Full transcript",
                "chat_title" => "AI Chat",
                "chat_desc" => "Ask about the video or improve the generated final text.",
                "chat_initial" => {
                    "Ask me about the video or use the button to improve the ready-to-paste text."
                }
                "you" => "You",
                "assistant" => "AI",
                "question" => "Question or instruction",
                "question_hint" => {
                    "Ex: improve the opening hook, make it more executive, answer a question about the video..."
                }
                "ask_ai" => "Ask AI",
                "improve_final" => "Improve final text",
                "status_need_url" => "You must paste a YouTube URL or ID.",
                "status_fetching" => "Fetching transcript and generating summary...",
                "status_context_ready" => {
                    "I already have the video context. You can ask me things or ask me to improve the final text."
                }
                "status_improved" => "Final text improved with AI.",
                "status_chat_ready" => "Chat response ready.",
                "status_loaded_from_history" => "Video loaded from history.",
                "models_loaded" => "Models loaded from Ollama",
                "status_need_transcript" => "You need to fetch the video transcript first.",
                "status_need_question" => "Write a question for the AI.",
                "status_consulting_ai" => "Consulting the AI...",
                "status_loading_models" => "Loading available models from Ollama...",
                "clipboard_ok" => "Final text copied to clipboard.",
                "models_noun" => "models",
                "summary" => "Summary",
                "key_points" => "Key Points",
                "language" => "Language",
                "type" => "Type",
                "output" => "Output",
                "transcript" => "Transcript",
                "characters" => "characters",
                "auto_subtitles" => "auto-generated subtitles",
                "manual_subtitles" => "manual subtitles",
                "no_key_points" => "No structured key points could be detected.",
                "chat_format" => "Ready-to-paste chat",
                "executive_format" => "Executive summary",
                "bullets_format" => "Direct bullets",
                "message" => "Message",
                "history" => "History",
                "import_history" => "Import YouTube history",
                "load" => "Load",
                "delete" => "Delete",
                "no_history" => "No saved videos",
                "history_imported" => "History imported",
                _ => key,
            },
            (
                Self::Pt,
                "app_desc"
                | "status_init"
                | "video"
                | "languages"
                | "model"
                | "reload_models"
                | "custom_model"
                | "format"
                | "run"
                | "processing"
                | "copy_final"
                | "video_details"
                | "video_config_desc"
                | "video_meta_desc"
                | "share_text"
                | "full_transcript"
                | "chat_title"
                | "chat_desc"
                | "chat_initial"
                | "you"
                | "assistant"
                | "question"
                | "question_hint"
                | "ask_ai"
                | "improve_final"
                | "status_need_url"
                | "status_fetching"
                | "status_context_ready"
                | "status_improved"
                | "status_chat_ready"
                | "status_loaded_from_history"
                | "models_loaded"
                | "status_need_transcript"
                | "status_need_question"
                | "status_consulting_ai"
                | "status_loading_models"
                | "clipboard_ok"
                | "models_noun"
                | "summary"
                | "key_points"
                | "language"
                | "type"
                | "output"
                | "transcript"
                | "characters"
                | "auto_subtitles"
                | "manual_subtitles"
                | "no_key_points"
                | "chat_format"
                | "executive_format"
                | "bullets_format"
                | "message",
            ) => match key {
                "app_desc" => {
                    "App nativo em Rust. Busca a transcrição, gera resumo com Ollama e monta texto pronto para compartilhar."
                }
                "status_init" => "Cole uma URL ou ID do YouTube e gere um resumo.",
                "video" => "Vídeo",
                "languages" => "Idiomas",
                "model" => "Modelo",
                "reload_models" => "Recarregar modelos do Ollama",
                "custom_model" => "modelo personalizado",
                "format" => "Formato",
                "run" => "Buscar transcrição e resumir",
                "processing" => "Processando...",
                "copy_final" => "📋 Copiar texto final",
                "video_details" => "Detalhes do vídeo",
                "video_config_desc" => {
                    "Configure o vídeo, o modelo e o tipo de saída antes de iniciar a análise."
                }
                "video_meta_desc" => "Metadados úteis do vídeo e da transcrição detectada.",
                "share_text" => "Texto pronto para colar",
                "full_transcript" => "Transcrição completa",
                "chat_title" => "Chat IA",
                "chat_desc" => "Pergunte sobre o vídeo ou melhore o texto final já gerado.",
                "chat_initial" => {
                    "Pergunte sobre o vídeo ou use o botão para melhorar o texto pronto para colar."
                }
                "you" => "Você",
                "assistant" => "IA",
                "question" => "Pergunta ou instrução",
                "question_hint" => {
                    "Ex: melhore o gancho inicial, deixe mais executivo, responda uma dúvida sobre o vídeo..."
                }
                "ask_ai" => "Perguntar à IA",
                "improve_final" => "Melhorar texto final",
                "status_need_url" => "Você deve colar uma URL ou ID do YouTube.",
                "status_fetching" => "Buscando transcrição e gerando resumo...",
                "status_context_ready" => {
                    "Já tenho o contexto do vídeo. Você pode me perguntar coisas ou pedir para melhorar o texto final."
                }
                "status_improved" => "Texto final melhorado com IA.",
                "status_chat_ready" => "Resposta do chat pronta.",
                "status_loaded_from_history" => "Vídeo carregado do histórico.",
                "models_loaded" => "Modelos carregados do Ollama",
                "status_need_transcript" => "Primeiro você precisa obter a transcrição do vídeo.",
                "status_need_question" => "Escreva uma pergunta para a IA.",
                "status_consulting_ai" => "Consultando a IA...",
                "status_loading_models" => "Carregando modelos disponíveis do Ollama...",
                "clipboard_ok" => "Texto final copiado para a área de transferência.",
                "models_noun" => "modelos",
                "summary" => "Resumo",
                "key_points" => "Pontos Importantes",
                "language" => "Idioma",
                "type" => "Tipo",
                "output" => "Saída",
                "transcript" => "Transcrição",
                "characters" => "caracteres",
                "auto_subtitles" => "legendas automáticas",
                "manual_subtitles" => "legendas manuais",
                "no_key_points" => "Não foi possível detectar pontos importantes estruturados.",
                "chat_format" => "Chat pronto para colar",
                "executive_format" => "Resumo executivo",
                "bullets_format" => "Bullets diretos",
                "message" => "Mensagem",
                "history" => "Histórico",
                "import_history" => "Importar histórico do YouTube",
                "load" => "Carregar",
                "delete" => "Excluir",
                "no_history" => "Nenhum vídeo salvo",
                "history_imported" => "Histórico importado",
                _ => key,
            },
            (Self::Fr | Self::De | Self::Ja | Self::ZhHans | Self::Ru | Self::Ar | Self::Hi, _) => {
                Self::En.text(key)
            }
            _ => key,
        }
    }
}
