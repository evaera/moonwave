import React from "react"
import styles from "./styles.module.css"

export default function SourceLink({ to }) {
  return (
    <a className={styles.sourceButton} href={to}>
      <div className={styles.sourceButtonText}>{`</>`}</div>
    </a>
  )
}
