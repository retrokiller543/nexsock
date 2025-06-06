import {createElement, Fragment} from './jsx-utils';

// Make createElement and Fragment global for JSX
declare global {
    var createElement: typeof import('./jsx-utils').createElement;
    var Fragment: typeof import('./jsx-utils').Fragment;
    const createServiceCard: (name: string, status: string, port: number) => JSX.Element;
}

globalThis.createElement = createElement;
globalThis.Fragment = Fragment;
globalThis.createServiceCard = createServiceCard;

// Example TSX component
function ServiceCard({ name, status, port }: { name: string; status: string; port: number }) {
    return (
        <div className="service-card">
            <h3>{name}</h3>
            <p>Status: <span className={`status-${status}`}>{status}</span></p>
            <p>Port: {port}</p>
        </div>
    );
}

export function createServiceCard(name: string, status: string, port: number): JSX.Element {
    return ServiceCard({ name, status, port }) as JSX.Element;
}