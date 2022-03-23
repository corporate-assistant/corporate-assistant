NLU is module that convert various phrases meant the same thing into single one that
is then used to raise a specific command.

### Working 
1. Strip some words from recognized command e.g. "i want to" or "the " "my "
2. Find stripped phrase among mappings.
3. Return chosen value of mapping e.g. target phrase

### design 
1. It should be that target value e.g. phrase mapped to is well recognizable by
deepspeech model.
2. When adding mappings do not register input phrase with words that are stripped anyway e.g. 
 do not register "i want my holidays" as key because "i want " will be removed. 
