import clsx from "clsx"
import React from "react"
import styles from "./HomepageFeatures.module.css"

const FeatureList = [
  {
    title: "Easy to Use",
    description: (
      <>
        You can generate a website with little-to-no configuration and just a
        few comments in your Lua code.
      </>
    ),
  },
  {
    title: "Write your docs alongside your source code",
    description: (
      <>
        With Moonwave, you can keep all of your code documentation in the same
        files as your code, so they are always up to date.
      </>
    ),
  },
  {
    title: "Moonwave is Modular",
    description: (
      <>
        Moonwave is three things: a plugin for{" "}
        <a href="https://docusaurus.io/" target="_blank">
          Docusaurus
        </a>
        , a command line tool, and a parser, which generates JSON from your doc
        comments, which can also be consumed by many different tools.
      </>
    ),
  },
]

function Feature({ Svg, title, description }) {
  return (
    <div className={clsx("col col--4")}>
      <div className="text--center">
        {Svg && <Svg className={styles.featureSvg} alt={title} />}
      </div>
      <div className=" padding-horiz--md">
        <h3 className="text--center">{title}</h3>
        <p>{description}</p>
      </div>
    </div>
  )
}

export default function HomepageFeatures() {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  )
}
