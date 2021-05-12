
## Instalation
### Linux
1. Download native client of deepspeech : https://github.com/mozilla/DeepSpeech/releases
2. Download/train model and scorer of deepspeech (https://github.com/mozilla/DeepSpeech/releases)
3. Build transcriber: __LIBRARY_PATH=&lt;path to native client&gt; cargo build --release__
4. Run transcriber: __LD_LIBRARY_PATH=&lt;path to native client&gt; ./target/release/corporate-assistant
    --model &lt;path to deepspeech model to use&gt; --scorer &lt;path to scorer (optionally)&gt;__

