import { createElement, type ReactNode } from 'react';

interface MarkdownDocumentProps {
    markdown: string;
}

function inlineMarkdown(text: string): ReactNode[] {
    const parts = text.split(/(`[^`]+`|\*\*[^*]+\*\*)/g);
    return parts.filter(Boolean).map((part, index) => {
        if (part.startsWith('`') && part.endsWith('`')) {
            return <code key={index}>{part.slice(1, -1)}</code>;
        }
        if (part.startsWith('**') && part.endsWith('**')) {
            return <strong key={index}>{part.slice(2, -2)}</strong>;
        }
        return part;
    });
}

function isBlockStart(line: string): boolean {
    const trimmed = line.trim();
    return trimmed === ''
        || /^#{1,6}\s/.test(trimmed)
        || /^```/.test(trimmed)
        || /^(---|___|\*\*\*)$/.test(trimmed)
        || /^>\s?/.test(trimmed)
        || /^[-*+]\s+/.test(trimmed)
        || /^\d+\.\s+/.test(trimmed)
        || /^ARCH_(?:NODE|DEPENDENCY):/.test(trimmed);
}

export function MarkdownDocument({ markdown }: MarkdownDocumentProps) {
    const lines = markdown.replace(/\r\n/g, '\n').split('\n');
    const blocks: ReactNode[] = [];
    let index = 0;

    while (index < lines.length) {
        const raw = lines[index];
        const trimmed = raw.trim();

        if (!trimmed) {
            index += 1;
            continue;
        }

        if (trimmed.startsWith('```')) {
            const language = trimmed.slice(3).trim();
            const code: string[] = [];
            index += 1;
            while (index < lines.length && !lines[index].trim().startsWith('```')) {
                code.push(lines[index]);
                index += 1;
            }
            if (index < lines.length) index += 1;
            blocks.push(
                <pre className="markdown-code" key={`code:${index}`}>
                    {language ? <span className="markdown-code-language">{language}</span> : null}
                    <code>{code.join('\n')}</code>
                </pre>,
            );
            continue;
        }

        const heading = trimmed.match(/^(#{1,6})\s+(.+)$/);
        if (heading) {
            const level = Math.min(heading[1].length + 1, 6);
            blocks.push(createElement(`h${level}`, { key: `heading:${index}` }, inlineMarkdown(heading[2])));
            index += 1;
            continue;
        }

        if (/^(---|___|\*\*\*)$/.test(trimmed)) {
            blocks.push(<hr key={`hr:${index}`} />);
            index += 1;
            continue;
        }

        if (/^ARCH_(?:NODE|DEPENDENCY):/.test(trimmed)) {
            const [marker, ...value] = trimmed.split(':');
            blocks.push(
                <div className="architecture-marker" key={`marker:${index}`}>
                    <span>{marker}</span>
                    <strong>{value.join(':').trim()}</strong>
                </div>,
            );
            index += 1;
            continue;
        }

        if (/^>\s?/.test(trimmed)) {
            const quote: string[] = [];
            while (index < lines.length && /^>\s?/.test(lines[index].trim())) {
                quote.push(lines[index].trim().replace(/^>\s?/, ''));
                index += 1;
            }
            blocks.push(<blockquote key={`quote:${index}`}>{inlineMarkdown(quote.join(' '))}</blockquote>);
            continue;
        }

        const unordered = trimmed.match(/^[-*+]\s+(.+)$/);
        if (unordered) {
            const items: string[] = [];
            while (index < lines.length) {
                const match = lines[index].trim().match(/^[-*+]\s+(.+)$/);
                if (!match) break;
                items.push(match[1]);
                index += 1;
            }
            blocks.push(
                <ul key={`ul:${index}`}>{items.map((item, itemIndex) => <li key={itemIndex}>{inlineMarkdown(item)}</li>)}</ul>,
            );
            continue;
        }

        const ordered = trimmed.match(/^\d+\.\s+(.+)$/);
        if (ordered) {
            const items: string[] = [];
            while (index < lines.length) {
                const match = lines[index].trim().match(/^\d+\.\s+(.+)$/);
                if (!match) break;
                items.push(match[1]);
                index += 1;
            }
            blocks.push(
                <ol key={`ol:${index}`}>{items.map((item, itemIndex) => <li key={itemIndex}>{inlineMarkdown(item)}</li>)}</ol>,
            );
            continue;
        }

        const paragraph = [trimmed];
        index += 1;
        while (index < lines.length && !isBlockStart(lines[index])) {
            paragraph.push(lines[index].trim());
            index += 1;
        }
        blocks.push(<p key={`p:${index}`}>{inlineMarkdown(paragraph.join(' '))}</p>);
    }

    return <article className="markdown-document">{blocks}</article>;
}
