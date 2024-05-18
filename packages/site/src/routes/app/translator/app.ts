import { App } from '@omujs/omu';
import { Identifier } from '@omujs/omu/identifier.js';
import type { TagKey } from '../category.js';
import thumbnail from './thumbnail.png';

export const IDENTIFIER = new Identifier('com.omuapps', 'translator');

export default function getApp(origin: string) {
    return new App(IDENTIFIER, {
        url: origin + '/app/translator',
        metadata: {
            locale: 'en',
            name: {
                en: 'Translator',
                ja: '翻訳',
            },
            description: {
                en: 'Translate messages.',
                ja: 'メッセージを翻訳します。',
            },
            icon: 'language-hiragana',
            image: thumbnail,
            tags: ['tool'] as TagKey[],
        },
    });
}
