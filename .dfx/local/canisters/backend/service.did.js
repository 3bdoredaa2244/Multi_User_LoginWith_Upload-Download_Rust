export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'check_file_exists' : IDL.Func([IDL.Text], [IDL.Bool], ['query']),
    'delete_file' : IDL.Func([IDL.Text], [IDL.Bool], []),
    'get_file_chunk' : IDL.Func(
        [IDL.Text, IDL.Nat64],
        [IDL.Opt(IDL.Vec(IDL.Nat8))],
        ['query'],
      ),
    'get_file_type' : IDL.Func([IDL.Text], [IDL.Opt(IDL.Text)], ['query']),
    'get_files' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text, IDL.Nat64))],
        ['query'],
      ),
    'get_total_chunks' : IDL.Func([IDL.Text], [IDL.Nat64], ['query']),
    'upload_file_chunk' : IDL.Func(
        [IDL.Text, IDL.Vec(IDL.Nat8), IDL.Nat64, IDL.Text],
        [],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
