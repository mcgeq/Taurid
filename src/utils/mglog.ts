import { invoke } from '@tauri-apps/api/core';

export enum LgLevel {
  T,
  D,
  E,
  I,
  W,
}

export const lg = async (level: LgLevel, msg: string, prefix: string = 'Front') => {
  await invoke('logmg', { lg_type: level.toString(), prefix: prefix, msg: msg });
};
