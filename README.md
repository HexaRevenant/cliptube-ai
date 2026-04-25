# 🎬 ClipTube AI

<div align="center">

**Turn YouTube videos into transcripts, summaries, key points and ready-to-share text.**

![Rust](https://img.shields.io/badge/Rust-stable-orange?style=for-the-badge&logo=rust)
![Desktop App](https://img.shields.io/badge/Desktop-Linux%20%7C%20macOS%20%7C%20Windows-blue?style=for-the-badge)
![Ollama](https://img.shields.io/badge/AI-Ollama-black?style=for-the-badge)
![Release](https://img.shields.io/badge/Release-AppImage-success?style=for-the-badge)
![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)

**🌍 Language / Idioma / Idioma:** [🇺🇸 English](#-english) · [🇪🇸 Español](#-español) · [🇧🇷 Português](#-português)

</div>

---

## 🇺🇸 English

### ✨ What is ClipTube AI?

**ClipTube AI** is a native desktop app built with **Rust + eframe/egui** that helps you transform YouTube videos into useful content you can **read, refine and share fast**.

### 🚀 What can it do?

- 📺 Paste a YouTube URL or video ID
- 📝 Fetch the **full transcript**
- 🤖 Generate an **AI summary**
- 🔎 Extract **key points**
- 💬 Build **ready-to-paste text**
- 🧠 Chat with AI about the video
- ✨ Improve the final text before sharing
- 🎛 Select models directly from **Ollama**
- 🔁 Retry pending/failed videos in bulk
- 🧰 Reprocess summaries from stored transcripts
- ⏱ Reprocess missing transcript timestamps
- 🛟 Use **yt-dlp fallback** when YouTube transcript tracks fail

### 🧩 Main features

- 🦀 Native desktop app in Rust
- ⚡ Local **Ollama** integration
- 🎯 Dynamic model selector loaded from Ollama
- 📌 Persistent right-side **AI chat** panel
- 🛟 Dual transcript strategy: internal YouTube captions + `yt-dlp` fallback
- 🔎 Full-text search indexes (FTS5) for videos and transcript segments
- 🔁 Recovery actions for failed/pending summary jobs
- 🌐 Base multilingual support
- 🖥 System language auto-detection
- 📦 Designed for **Linux / macOS / Windows**

### 📋 Requirements

- Rust / Cargo
- Ollama running locally
- `yt-dlp` installed and available in PATH (for transcript fallback)
- At least one model available in `ollama list`
- A graphical desktop environment

### 🔧 Environment variables

- `OLLAMA_MODEL` → default model
- `OLLAMA_CHAT_URL` → Ollama chat endpoint
- `MAX_TRANSCRIPT_CHARS_FOR_AI` → max chars sent to AI
- `OLLAMA_CHUNK_SIZE` → chunk size for long transcripts
- `OLLAMA_MAX_CHUNKS` → max number of chunks

### 🛠 Run

#### Development

```bash
cargo run
```

#### Release

```bash
cargo run --release
```


### 🌐 Website

A GitHub Pages landing is prepared under `docs/` and can be deployed with the `Pages` workflow.
The repository URLs are already wired to the public GitHub repository in `docs/script.js`.

The landing now also includes:

- `robots.txt`
- `sitemap.xml`
- Open Graph / Twitter metadata
- structured data (`SoftwareApplication` + `FAQPage`)
- a dedicated social preview image at `docs/assets/og-image.png`

### 🖼 Localized screenshots

The landing can load app screenshots per language and fall back to English automatically.

- Save screenshots in `docs/assets/screenshots/`
- Recommended names:
  - `app-preview-en.png`
  - `app-preview-es.png`
  - `app-preview-pt.png`
  - `app-preview-fr.png`
  - `app-preview-de.png`
  - `app-preview-ja.png`
  - `app-preview-zh-Hans.png`
  - `app-preview-ru.png`
  - `app-preview-ar.png`
  - `app-preview-hi.png`
- If a language-specific image is missing, the site uses `app-preview-en.png`
- If the English image is also missing, it falls back to `docs/assets/app-preview.svg`

To open the desktop app in a specific UI language for screenshots:

```bash
scripts/run-ui-lang.sh en
scripts/run-ui-lang.sh pt
```

Note: the desktop app now loads Noto fallback fonts for Arabic, Hindi, Chinese and Japanese on systems where those fonts exist (for example most Linux systems with Noto installed). Arabic also gets best-effort right-aligned UI fields, but full complex-script shaping still depends on what `egui` can render.

### 🧪 CI / Releases

This repository includes GitHub Actions workflows for:

- ✅ CI on **Linux / macOS / Windows**
- 📦 Release builds on tags like `v1.0.0`
- 🐧 Linux: AppImage + versioned `.tar.gz`
- 🍎 macOS: Intel, Apple Silicon and Universal builds (`.dmg` + `.app` ZIP)
- 🪟 Windows: ZIP package with `cliptube-ai.exe` and embedded icon resources

Artifacts are generated automatically when pushing tags starting with `v`.

### 🐧 Linux AppImage note

On Ubuntu and some other Linux desktops, downloaded AppImages usually need executable permission before the first launch:

```bash
chmod +x cliptube-ai-linux-x86_64.AppImage
./cliptube-ai-linux-x86_64.AppImage
```

### 📄 License

This project is released under the **MIT License**. See [`LICENSE`](LICENSE).

---

## 🇪🇸 Español

### ✨ ¿Qué es ClipTube AI?

**ClipTube AI** es una app de escritorio nativa en **Rust + eframe/egui** que transforma videos de YouTube en contenido útil para **leer, editar y compartir rápido**.

### 🚀 ¿Qué puede hacer?

- 📺 Pegar una URL o ID de YouTube
- 📝 Obtener la **transcripción completa**
- 🤖 Generar un **resumen con IA**
- 🔎 Extraer **puntos importantes**
- 💬 Crear un **texto listo para copiar y pegar**
- 🧠 Chatear con la IA sobre el video
- ✨ Mejorar el texto final antes de compartirlo
- 🎛 Elegir modelos directamente desde **Ollama**
- 🔁 Reintentar en lote los videos pendientes/fallidos
- 🧰 Reprocesar resúmenes desde transcripts guardados
- ⏱ Reprocesar tiempos faltantes de transcript
- 🛟 Usar **fallback con yt-dlp** cuando fallen las pistas de subtítulos de YouTube

### 🧩 Características principales

- 🦀 App nativa de escritorio en Rust
- ⚡ Integración con **Ollama local**
- 🎯 Selector dinámico de modelos cargado desde Ollama
- 📌 Panel lateral fijo de **chat IA**
- 🛟 Estrategia dual de transcript: captions internas de YouTube + fallback `yt-dlp`
- 🔎 Índices de búsqueda full-text (FTS5) para videos y segmentos
- 🔁 Acciones de recuperación para jobs de resumen pendientes/fallidos
- 🌐 Soporte base multiidioma
- 🖥 Detección automática del idioma del sistema
- 📦 Pensada para **Linux / macOS / Windows**

### 📋 Requisitos

- Rust / Cargo
- Ollama ejecutándose localmente
- `yt-dlp` instalado y disponible en PATH (para fallback de transcript)
- Al menos un modelo disponible en `ollama list`
- Un entorno gráfico de escritorio

### 🔧 Variables de entorno

- `OLLAMA_MODEL` → modelo por defecto
- `OLLAMA_CHAT_URL` → endpoint de chat de Ollama
- `MAX_TRANSCRIPT_CHARS_FOR_AI` → máximo de caracteres enviados a la IA
- `OLLAMA_CHUNK_SIZE` → tamaño por bloque para transcripts largos
- `OLLAMA_MAX_CHUNKS` → cantidad máxima de bloques

### 🛠 Ejecutar

#### Desarrollo

```bash
cargo run
```

#### Release

```bash
cargo run --release
```

### 🌐 Sitio web

Hay una landing para GitHub Pages preparada en `docs/` y se puede publicar con el workflow `Pages`.
Las URLs del repositorio ya están conectadas al repo público en `docs/script.js`.

La landing ahora también incluye:

- `robots.txt`
- `sitemap.xml`
- metadata Open Graph / Twitter
- structured data (`SoftwareApplication` + `FAQPage`)
- imagen social dedicada en `docs/assets/og-image.png`

### 🖼 Capturas localizadas

La landing puede cargar capturas por idioma y usar inglés como fallback automático.

- Guarda las capturas en `docs/assets/screenshots/`
- Nombres recomendados:
  - `app-preview-en.png`
  - `app-preview-es.png`
  - `app-preview-pt.png`
  - `app-preview-fr.png`
  - `app-preview-de.png`
  - `app-preview-ja.png`
  - `app-preview-zh-Hans.png`
  - `app-preview-ru.png`
  - `app-preview-ar.png`
  - `app-preview-hi.png`
- Si falta la imagen del idioma activo, la web usa `app-preview-en.png`
- Si también falta la de inglés, cae a `docs/assets/app-preview.svg`

Para abrir la app en un idioma específico y sacar capturas:

```bash
scripts/run-ui-lang.sh en
scripts/run-ui-lang.sh pt
```

Nota: la app desktop ahora carga fuentes Noto de fallback para árabe, hindi, chino y japonés en sistemas donde esas fuentes existan (por ejemplo Linux con Noto instalado). En árabe también apliqué alineación derecha en campos clave, aunque el shaping completo de escritura compleja sigue dependiendo de lo que `egui` pueda renderizar.

### 🧪 CI / Releases

Este repositorio incluye GitHub Actions para:

- ✅ CI en **Linux / macOS / Windows**
- 📦 Builds de release con tags como `v1.0.0`
- 🐧 Linux: AppImage + `.tar.gz` versionado
- 🍎 macOS: builds Intel, Apple Silicon y Universal (`.dmg` + bundle `.app` en ZIP)
- 🪟 Windows: ZIP con `cliptube-ai.exe` e icono incrustado

Los artefactos se generan automáticamente al subir tags que empiezan con `v`.

### 🐧 Nota sobre AppImage en Linux

En Ubuntu y otros escritorios Linux, los AppImage descargados suelen necesitar permiso de ejecución antes del primer inicio:

```bash
chmod +x cliptube-ai-linux-x86_64.AppImage
./cliptube-ai-linux-x86_64.AppImage
```

### 📄 Licencia

Este proyecto se publica bajo la **Licencia MIT**. Revisa [`LICENSE`](LICENSE).

---

## 🇧🇷 Português

### ✨ O que é o ClipTube AI?

**ClipTube AI** é um aplicativo desktop nativo feito com **Rust + eframe/egui** que transforma vídeos do YouTube em conteúdo útil para **ler, refinar e compartilhar rapidamente**.

### 🚀 O que ele faz?

- 📺 Colar uma URL ou ID do YouTube
- 📝 Obter a **transcrição completa**
- 🤖 Gerar um **resumo com IA**
- 🔎 Extrair **pontos importantes**
- 💬 Criar um **texto pronto para copiar e colar**
- 🧠 Conversar com a IA sobre o vídeo
- ✨ Melhorar o texto final antes de compartilhar
- 🎛 Escolher modelos diretamente do **Ollama**
- 🔁 Repetir em lote vídeos pendentes/com falha
- 🧰 Reprocessar resumos a partir de transcrições já salvas
- ⏱ Reprocessar timestamps ausentes dos segmentos
- 🛟 Usar **fallback com yt-dlp** quando falhar a trilha de legendas do YouTube

### 🧩 Principais recursos

- 🦀 App desktop nativo em Rust
- ⚡ Integração com **Ollama local**
- 🎯 Seletor dinâmico de modelos carregado do Ollama
- 📌 Painel lateral fixo de **chat com IA**
- 🛟 Estratégia dupla de transcript: captions internas do YouTube + fallback `yt-dlp`
- 🔎 Índices full-text (FTS5) para vídeos e segmentos da transcrição
- 🔁 Ações de recuperação para jobs de resumo pendentes/com falha
- 🌐 Suporte base multilíngue
- 🖥 Detecção automática do idioma do sistema
- 📦 Projetado para **Linux / macOS / Windows**

### 📋 Requisitos

- Rust / Cargo
- Ollama rodando localmente
- `yt-dlp` instalado e disponível no PATH (para fallback de transcrição)
- Pelo menos um modelo disponível em `ollama list`
- Um ambiente gráfico de desktop

### 🔧 Variáveis de ambiente

- `OLLAMA_MODEL` → modelo padrão
- `OLLAMA_CHAT_URL` → endpoint de chat do Ollama
- `MAX_TRANSCRIPT_CHARS_FOR_AI` → máximo de caracteres enviados para a IA
- `OLLAMA_CHUNK_SIZE` → tamanho por bloco para transcrições longas
- `OLLAMA_MAX_CHUNKS` → número máximo de blocos

### 🛠 Executar

#### Desenvolvimento

```bash
cargo run
```

#### Release

```bash
cargo run --release
```

### 🌐 Website

Há uma landing para GitHub Pages preparada em `docs/` e ela pode ser publicada com o workflow `Pages`.
As URLs do repositório já estão ligadas ao repositório público em `docs/script.js`.

A landing agora também inclui:

- `robots.txt`
- `sitemap.xml`
- metadata Open Graph / Twitter
- structured data (`SoftwareApplication` + `FAQPage`)
- imagem social dedicada em `docs/assets/og-image.png`

### 🖼 Capturas localizadas

A landing pode carregar capturas por idioma e usar inglês como fallback automático.

- Salve as capturas em `docs/assets/screenshots/`
- Nomes recomendados:
  - `app-preview-en.png`
  - `app-preview-es.png`
  - `app-preview-pt.png`
  - `app-preview-fr.png`
  - `app-preview-de.png`
  - `app-preview-ja.png`
  - `app-preview-zh-Hans.png`
  - `app-preview-ru.png`
  - `app-preview-ar.png`
  - `app-preview-hi.png`
- Se faltar a imagem do idioma ativo, o site usa `app-preview-en.png`
- Se a imagem em inglês também faltar, ele cai para `docs/assets/app-preview.svg`

Para abrir o app em um idioma específico e tirar capturas:

```bash
scripts/run-ui-lang.sh en
scripts/run-ui-lang.sh pt
```

Nota: o app desktop agora carrega fontes Noto de fallback para árabe, hindi, chinês e japonês em sistemas onde essas fontes existirem (por exemplo Linux com Noto instalado). Para árabe também apliquei alinhamento à direita em campos principais, embora o shaping completo de escrita complexa ainda dependa do que o `egui` consegue renderizar.

### 🧪 CI / Releases

Este repositório inclui GitHub Actions para:

- ✅ CI em **Linux / macOS / Windows**
- 📦 Builds de release com tags como `v1.0.0`
- 🐧 Linux: AppImage + `.tar.gz` versionado
- 🍎 macOS: builds Intel, Apple Silicon e Universal (`.dmg` + bundle `.app` em ZIP)
- 🪟 Windows: ZIP com `cliptube-ai.exe` e ícone embutido

Os artefatos são gerados automaticamente ao enviar tags que começam com `v`.

### 🐧 Nota sobre AppImage no Linux

No Ubuntu e em outros desktops Linux, AppImages baixados normalmente precisam de permissão de execução antes do primeiro uso:

```bash
chmod +x cliptube-ai-linux-x86_64.AppImage
./cliptube-ai-linux-x86_64.AppImage
```

### 📄 Licença

Este projeto é distribuído sob a **Licença MIT**. Veja [`LICENSE`](LICENSE).
