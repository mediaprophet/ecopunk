from pathlib import Path
base=Path(__file__).parent
file=base/'candidate/vibe/src/eval/mod.rs'
text=file.read_text(encoding='utf8')
names=['crypto_generate_key_dispatches_to_host','crypto_sign_dispatches_to_host',
'crypto_sign_needs_string_args','t24_dag_execute_dispatches_to_host',
't42_hid_poll_dispatches_to_host','t42_hid_wait_dispatches_to_host',
't45_cue_post_dispatches_to_host','t45_cue_post_needs_string_id']
for name in names:
    start=text.index('    fn '+name+'()')
    end=text.find('    #[test]',start)
    if end<0: end=len(text)
    block=text[start:end]
    assert block.count('            fn main()')==1,name
    block=block.replace('            fn main()', '            effect fn main()',1)
    text=text[:start]+block+text[end:]
file.write_text(text,encoding='utf8',newline='\n')
print('8 host dispatch/type fixtures explicitly declare External effect; expected host errors preserved')
