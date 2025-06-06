import css from '../generated/components/NsButton.css';

interface NsButtonProps {
  children?: any;
  variant?: 'primary' | 'secondary' | 'danger' | 'ghost';
  size?: 'small' | 'medium' | 'large';
  disabled?: boolean;
  loading?: boolean;
  type?: 'button' | 'submit' | 'reset';
  onClick?: () => void;
  href?: string;
  target?: string;
  className?: string;
}

/**
 * Nexsock UI Button Component
 * 
 * Usage in templates:
 * <ns-button variant="primary" size="large">Click me</ns-button>
 * <ns-button variant="danger" onclick="alert('Danger!')">Delete</ns-button>
 */
function NsButtonComponent(props: NsButtonProps): JSX.Element {
  const {
    children,
    variant = 'primary',
    size = 'medium',
    disabled = false,
    loading = false,
    type = 'button',
    onClick,
    href,
    target,
    className = ''
  } = props;

  const classes = [
    'ns-button',
    variant,
    size,
    loading && 'loading',
    className
  ].filter(Boolean).join(' ');

  // If href is provided, render as link
  if (href) {
    return (
      <a
        href={href}
        target={target}
        className={classes}
        onClick={onClick}
        aria-disabled={disabled}
      >
        {children}
      </a>
    );
  }

  // Otherwise render as button
  return (
    <button
      type={type}
      className={classes}
      {...disabled ? { disabled: true } : {}}
      onClick={onClick}
    >
      {children}
    </button>
  );
}

// Attach CSS
NsButtonComponent.css = css;

export const NsButton = NsButtonComponent;