import css from '../generated/components/NsBadge.css';

interface NsBadgeProps {
  children?: any;
  variant?: 'success' | 'warning' | 'error' | 'info' | 'neutral';
  size?: 'small' | 'medium' | 'large';
  style?: 'default' | 'outline' | 'solid';
  className?: string;
}

/**
 * Nexsock UI Badge Component
 * 
 * Usage in templates:
 * <ns-badge variant="success">Running</ns-badge>
 * <ns-badge variant="error" style="solid">Failed</ns-badge>
 */
function NsBadgeComponent(props: NsBadgeProps): JSX.Element {
  const {
    children,
    variant = 'neutral',
    size = 'medium',
    style = 'default',
    className = ''
  } = props;

  const classes = [
    'ns-badge',
    variant,
    size !== 'medium' && size,
    style !== 'default' && style,
    className
  ].filter(Boolean).join(' ');

  return (
    <span className={classes}>
      {children}
    </span>
  );
}

// Attach CSS
NsBadgeComponent.css = css;

export const NsBadge = NsBadgeComponent;