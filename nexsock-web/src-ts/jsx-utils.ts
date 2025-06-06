// Simple JSX factory functions for standalone JSX without React
export function createElement(
    tag: string | Function,
    props: Record<string, any> | null,
    ...children: any[]
): HTMLElement | DocumentFragment {
    if (typeof tag === 'function') {
        return tag({ ...props, children });
    }

    const element = document.createElement(tag);
    
    if (props) {
        Object.entries(props).forEach(([key, value]) => {
            if (key === 'className') {
                element.className = value;
            } else if (key.startsWith('on') && typeof value === 'function') {
                const event = key.toLowerCase().slice(2);
                element.addEventListener(event, value);
            } else {
                element.setAttribute(key, value);
            }
        });
    }

    children.flat().forEach(child => {
        if (typeof child === 'string' || typeof child === 'number') {
            element.appendChild(document.createTextNode(String(child)));
        } else if (child instanceof Node) {
            element.appendChild(child);
        }
    });

    return element;
}

export function Fragment({ children }: { children: any[] }): DocumentFragment {
    const fragment = document.createDocumentFragment();
    children.flat().forEach(child => {
        if (typeof child === 'string' || typeof child === 'number') {
            fragment.appendChild(document.createTextNode(String(child)));
        } else if (child instanceof Node) {
            fragment.appendChild(child);
        }
    });
    return fragment;
}