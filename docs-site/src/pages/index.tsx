import type { ReactNode } from "react";
import clsx from "clsx";
import Link from "@docusaurus/Link";
import useDocusaurusContext from "@docusaurus/useDocusaurusContext";
import Layout from "@theme/Layout";
import HomepageFeatures from "@site/src/components/HomepageFeatures";
import Heading from "@theme/Heading";

import styles from "./index.module.css";

function HomepageHeader() {
  const { siteConfig } = useDocusaurusContext();
  return (
    <header className={styles.heroBanner}>
      <div className={styles.worldMapBackground}></div>
      <div className={clsx("container", styles.heroContainer)}>
        <div className={styles.heroContent}>
          <div className={styles.heroLogoSection}>
            <img
              src="img/catp2p_logo.svg"
              alt="CatP2P Logo"
              className={styles.heroLogo}
            />
          </div>
          <div className={styles.heroTextSection}>
            <Heading as="h1" className={styles.heroTitle}>
              <span className={styles.heroTitleMain}>CatP2P</span>
            </Heading>
            <p className={styles.heroSubtitle}>{siteConfig.tagline}</p>
            <div className={styles.heroButtons}>
              <Link
                className={clsx("button button--primary button--lg", styles.heroButton)}
                to="/docs/intro"
              >
                Get Started
              </Link>
              <Link
                className={clsx("button button--outline button--lg", styles.heroButton)}
                to="https://github.com/johnnyvillas/catp2p"
              >
                View on GitHub
              </Link>
            </div>
          </div>
        </div>
      </div>
    </header>
  );
}

export default function Home(): ReactNode {
  const { siteConfig } = useDocusaurusContext();
  return (
    <Layout
      title={`${siteConfig.title} - High-Performance P2P Library`}
      description="A high-performance P2P library for distributed computing"
    >
      <HomepageHeader />
      <main>
        <HomepageFeatures />
      </main>
    </Layout>
  );
}
