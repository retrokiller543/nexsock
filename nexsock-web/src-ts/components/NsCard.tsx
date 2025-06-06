import {css} from '../generated/components/NsCard.css';

interface NsCardProps {
  children?: any;
  title?: string;
  subtitle?: string;
  footer?: any;
  variant?: 'default' | 'elevated' | 'flat' | 'borderless';
  className?: string;
}

/**
 * Nexsock UI Card Component
 * 
 * Usage in templates:
 * <ns-card title="Service Status" subtitle="Current status of your services">
 *   <p>Card content goes here</p>
 * </ns-card>
 */
function NsCardComponent(props: NsCardProps): JSX.Element {
  const {
    children,
    title,
    subtitle,
    footer,
    variant = 'default',
    className = ''
  } = props;

  const classes = [
    'ns-card',
    variant !== 'default' && variant,
    className
  ].filter(Boolean).join(' ');

  return (
    <div className={classes}>
      {(title || subtitle) && (
        <div className="ns-card-header">
          {title && <h3 className="ns-card-title">{title}</h3>}
          {subtitle && <p className="ns-card-subtitle">{subtitle}</p>}
        </div>
      )}
      
      {children && (
        <div className="ns-card-body">
          {children}
        </div>
      )}
      
      {footer && (
        <div className="ns-card-footer">
          {footer}
        </div>
      )}
    </div>
  );
}

// Attach CSS
NsCardComponent.css = css;

export const NsCard = NsCardComponent;