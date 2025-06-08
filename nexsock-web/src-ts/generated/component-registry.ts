// Auto-generated component registry - do not edit manually
// This file is regenerated on every build

import {ErrorModal} from '../components/ErrorModal';
import {NsCard} from '../components/NsCard';
import {NsBadge} from '../components/NsBadge';
import {ErrorNotification} from '../components/ErrorNotification';
import {NsButton} from '../components/NsButton';

import {registerComponent} from '../core/web-components';

export const COMPONENT_REGISTRY = {
  'error-modal': ErrorModal,
  'ns-card': NsCard,
  'ns-badge': NsBadge,
  'error-notification': ErrorNotification,
  'ns-button': NsButton,
} as const;

export type ComponentTagName = keyof typeof COMPONENT_REGISTRY;

export function registerAllComponents(): void {
  Object.entries(COMPONENT_REGISTRY).forEach(([tagName, component]) => {
    registerComponent(tagName, component);
  });
  console.log('Auto-registered components:', Object.keys(COMPONENT_REGISTRY));
}
