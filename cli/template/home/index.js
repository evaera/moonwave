import Link from "@docusaurus/Link"
import useDocusaurusContext from "@docusaurus/useDocusaurusContext"
import Layout from "@theme/Layout"
import clsx from "clsx"
import React from "react"
import styles from "./index.module.css"

const FEATURES = /***features***/

  function Feature({ image, title, description }) {
    return (
      <div className={clsx("col col--4")}>
        {image && (
          <div className="text--center">
            <img className={styles.featureSvg} alt={title} src={image} />
          </div>
        )}
        <div className="text--center padding-horiz--md">
          <h3>{title}</h3>
          <p>{description}</p>
        </div>
      </div>
    )
  }

export function HomepageFeatures() {
  if (!FEATURES) return null

  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FEATURES.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  )
}

function HomepageHeader() {
  const { siteConfig } = useDocusaurusContext()
  const bannerImage = siteConfig.customFields.bannerImage
  const hasBannerImage = bannerImage ? true : false
  const heroBannerStyle = hasBannerImage ? { backgroundImage: `url("${bannerImage}")` } : null

  const titleClassName = clsx("hero__title", {
    [styles.titleOnBannerImage]: hasBannerImage
  })
  const taglineClassName = clsx("hero__subtitle", {
    [styles.taglineOnBannerImage]: hasBannerImage
  })

  return (
    <header className={clsx("hero", styles.heroBanner)} style={heroBannerStyle}>
      <div className="container">
        <h1 className={titleClassName}>{siteConfig.title}</h1>
        <p className={taglineClassName}>{siteConfig.tagline}</p>
        <div className={styles.buttons}>
          <Link
            className="button button--secondary button--lg"
            to="/docs/intro"
          >
            Get Started →
          </Link>
        </div>
      </div>
    </header>
  )
}

export default function Home() {
  const { siteConfig, tagline } = useDocusaurusContext()
  return (
    <Layout title={siteConfig.title} description={tagline}>
      <HomepageHeader />
      <main>
        <HomepageFeatures />
        <div className="container">{/***readme***/}</div>
      </main>
    </Layout>
  )
}
