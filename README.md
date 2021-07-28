
## Instalation
### Linux
##### Fedora 33
dnf install speech-dispatcher-devel SDL2-devel clang-devel pango-devel
##### Ubuntu 18.04
apt-get install -y speech-dispatcher libspeechd-dev libsdl2-dev libbpango1.0-dev libpangox-1.0-dev
#### Building
1. Download native client of deepspeech : https://github.com/mozilla/DeepSpeech/releases
2. Download/train model and scorer of deepspeech (https://github.com/mozilla/DeepSpeech/releases)
3. Build transcriber: __LIBRARY_PATH=&lt;path to native client&gt; cargo build --release__
4. Run transcriber: __LD_LIBRARY_PATH=&lt;path to native client&gt; ./target/release/corporate-assistant
    --model &lt;path to deepspeech model to use&gt; --scorer &lt;path to scorer (optionally)&gt;__

