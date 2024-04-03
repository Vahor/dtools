import { chatManager } from './channels';
import { readTranslationsFile } from './utils';

interface TranslationMap {
  [key: number]: string;
}

type Languages = 'fr';

let translations: TranslationMap | undefined = undefined;

export const loadTranslation = async (lang: Languages) => {
  const rawTranslation = (await readTranslationsFile(`i18n_${lang}.json`)) as Record<
    string,
    string
  >;
  translations = Object.entries(rawTranslation).reduce((acc, [key, value]) => {
    acc[parseInt(key)] = value;
    return acc;
  }, {} as TranslationMap);
  [chatManager].forEach((manager) => manager.load());
};

export const translate = (id: number) => {
  if (translations === undefined) {
    throw new Error('Translations not loaded');
  }

  return translations[id];
};
