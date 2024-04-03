import { exists, BaseDirectory, readTextFile } from '@tauri-apps/plugin-fs';

const EXTRACTOR_DIR = 'dofus/datafus';
const ENTITIES_DIR = 'entities_json';
const TRANSLATIONS_DIR = 'translations_json';

export const readEntitiesFile = async (filename: string): Promise<unknown> => {
  const path = `${EXTRACTOR_DIR}/${ENTITIES_DIR}/${filename}`;
  const exist = await exists(path, { baseDir: BaseDirectory.AppData });
  if (!exist) {
    throw new Error(`Entities file ${filename} does not exist`);
  }

  const content = await readTextFile(path, { baseDir: BaseDirectory.AppData });
  return JSON.parse(content);
};

export const readTranslationsFile = async (filename: string): Promise<unknown> => {
  const path = `${EXTRACTOR_DIR}/${TRANSLATIONS_DIR}/${filename}`;
  const exist = await exists(path, { baseDir: BaseDirectory.AppData });
  if (!exist) {
    throw new Error(`Translations file ${filename} does not exist`);
  }

  const content = await readTextFile(path, { baseDir: BaseDirectory.AppData });
  return JSON.parse(content);
};

export const readJsonlFile = async (filename: string): Promise<unknown> => {
  const path = `${filename}`;
  const exist = await exists(path, { baseDir: BaseDirectory.AppData });
  if (!exist) {
    throw new Error(`Jsonl file ${filename} does not exist`);
  }

  const content = await readTextFile(path, { baseDir: BaseDirectory.AppData });
  const lines = content.split('\n').filter(Boolean);
  return lines.map((line) => JSON.parse(line));
};
