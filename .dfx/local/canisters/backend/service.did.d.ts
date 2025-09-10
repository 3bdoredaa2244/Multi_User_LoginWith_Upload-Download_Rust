import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface FileMeta { 'size' : bigint, 'file_type' : string }
export interface _SERVICE {
  'check_file_exists' : ActorMethod<[string], boolean>,
  'delete_file' : ActorMethod<[string], boolean>,
  'get_file_chunk' : ActorMethod<
    [string, bigint],
    [] | [Uint8Array | number[]]
  >,
  'get_file_type' : ActorMethod<[string], [] | [string]>,
  'get_files' : ActorMethod<[], Array<[string, string, bigint]>>,
  'get_total_chunks' : ActorMethod<[string], bigint>,
  'upload_file_chunk' : ActorMethod<
    [string, Uint8Array | number[], bigint, string],
    undefined
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
