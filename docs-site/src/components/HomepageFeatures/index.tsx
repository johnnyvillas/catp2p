import type {ReactNode} from 'react';
import clsx from 'clsx';
import Heading from '@theme/Heading';
import styles from './styles.module.css';
// Import icons from react-icons
import { 
  FaMicrochip, 
  FaNetworkWired, 
  FaTasks, 
  FaDatabase, 
  FaChartBar, 
  FaShieldAlt,
  FaGlobe,
  FaMemory,
  FaHdd
} from 'react-icons/fa';

// Main features highlighted at the top
type MainFeatureItem = {
  title: string;
  Icon: React.ComponentType<React.ComponentProps<'svg'>>;
  description: ReactNode;
};

const MainFeatureList: MainFeatureItem[] = [
  {
    title: 'High-Performance Computing',
    Icon: FaMicrochip,
    description: (
      <>
        Leverage the full power of modern hardware with comprehensive 
        benchmarking and optimization for CPU, GPU, memory, and storage.
      </>
    ),
  },
  {
    title: 'P2P Networking',
    Icon: FaNetworkWired,
    description: (
      <>
        Connect devices globally with robust peer discovery and secure 
        communication built on modern libp2p technology.
      </>
    ),
  },
  {
    title: 'Distributed Task Management',
    Icon: FaTasks,
    description: (
      <>
        Efficiently distribute computational workloads with intelligent 
        resource allocation and dynamic load balancing.
      </>
    ),
  },
];

function MainFeature({title, Icon, description}: MainFeatureItem) {
  return (
    <div className="col col--4">
      <div className={styles.mainFeatureCard}>
        <div className={styles.mainFeatureIconWrapper}>
          <Icon className={styles.mainFeatureIcon} />
        </div>
        <div className={styles.mainFeatureContent}>
          <Heading as="h3">{title}</Heading>
          <p>{description}</p>
        </div>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): ReactNode {
  return (
    <>
      {/* Main Features Section */}
      <section className={styles.mainFeatures}>
        <div className="container">
          <div className={styles.mainFeatureHeader}>
            <Heading as="h2">Core Capabilities</Heading>
            <p>The foundation of CatP2P's distributed computing platform</p>
          </div>
          <div className="row">
            {MainFeatureList.map((props, idx) => (
              <MainFeature key={idx} {...props} />
            ))}
          </div>
        </div>
      </section>

      {/* Secondary Features Section */}
      <section className={styles.features}>
        <div className="container">
          <div className={styles.featureHeader}>
            <Heading as="h2">Additional Features</Heading>
            <p>A comprehensive suite of tools for building high-performance distributed applications</p>
          </div>
          
          <div className={styles.featureGrid}>
            {/* Row 1 */}
            <div className={styles.featureItem}>
              <div className={styles.featureCard}>
                <div className={styles.featureIconWrapper}>
                  <FaMemory className={styles.featureIcon} />
                </div>
                <div className={styles.featureContent}>
                  <Heading as="h3">Resource Management</Heading>
                  <p>Monitor and allocate resources dynamically based on workload requirements.</p>
                </div>
              </div>
            </div>
            
            <div className={clsx(styles.featureItem, styles.featureHighlight)}>
              <div className={styles.featureCard}>
                <div className={styles.featureIconWrapper}>
                  <FaChartBar className={styles.featureIcon} />
                </div>
                <div className={styles.featureContent}>
                  <Heading as="h3">Comprehensive Benchmarking</Heading>
                  <p>Assess node capabilities with detailed performance benchmarks.</p>
                </div>
              </div>
            </div>
            
            <div className={styles.featureItem}>
              <div className={styles.featureCard}>
                <div className={styles.featureIconWrapper}>
                  <FaHdd className={styles.featureIcon} />
                </div>
                <div className={styles.featureContent}>
                  <Heading as="h3">Persistent Storage</Heading>
                  <p>Local storage with efficient serialization and database integration.</p>
                </div>
              </div>
            </div>
            
            {/* Row 2 */}
            <div className={clsx(styles.featureItem, styles.featureHighlight)}>
              <div className={styles.featureCard}>
                <div className={styles.featureIconWrapper}>
                  <FaGlobe className={styles.featureIcon} />
                </div>
                <div className={styles.featureContent}>
                  <Heading as="h3">Global Network</Heading>
                  <p>Connect devices globally to unlock untapped computational potential.</p>
                </div>
              </div>
            </div>
            
            <div className={styles.featureItem}>
              <div className={styles.featureCard}>
                <div className={styles.featureIconWrapper}>
                  <FaDatabase className={styles.featureIcon} />
                </div>
                <div className={styles.featureContent}>
                  <Heading as="h3">Scoring System</Heading>
                  <p>Track contributions with a fair and transparent reward system.</p>
                </div>
              </div>
            </div>
            
            <div className={clsx(styles.featureItem, styles.featureHighlight)}>
              <div className={styles.featureCard}>
                <div className={styles.featureIconWrapper}>
                  <FaShieldAlt className={styles.featureIcon} />
                </div>
                <div className={styles.featureContent}>
                  <Heading as="h3">Secure Communications</Heading>
                  <p>Built with security in mind, featuring encrypted communications.</p>
                </div>
              </div>
            </div>
            
            {/* Row 3 - Add 3 more features to complete the 3x3 grid */}
            <div className={styles.featureItem}>
              <div className={styles.featureCard}>
                <div className={styles.featureIconWrapper}>
                  <FaMicrochip className={styles.featureIcon} />
                </div>
                <div className={styles.featureContent}>
                  <Heading as="h3">GPU Acceleration</Heading>
                  <p>Utilize GPU computing power for parallel processing tasks.</p>
                </div>
              </div>
            </div>
            
            <div className={clsx(styles.featureItem, styles.featureHighlight)}>
              <div className={styles.featureCard}>
                <div className={styles.featureIconWrapper}>
                  <FaNetworkWired className={styles.featureIcon} />
                </div>
                <div className={styles.featureContent}>
                  <Heading as="h3">Network Optimization</Heading>
                  <p>Intelligent routing and bandwidth management for efficient data transfer.</p>
                </div>
              </div>
            </div>
            
            <div className={styles.featureItem}>
              <div className={styles.featureCard}>
                <div className={styles.featureIconWrapper}>
                  <FaTasks className={styles.featureIcon} />
                </div>
                <div className={styles.featureContent}>
                  <Heading as="h3">Fault Tolerance</Heading>
                  <p>Robust error handling and recovery mechanisms for reliable operation.</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>
    </>
  );
}
