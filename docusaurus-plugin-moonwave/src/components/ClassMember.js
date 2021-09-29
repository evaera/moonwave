import Heading from "@theme/Heading"
import React from "react"
import Admonition from "./Admonition"
import Badge from "./Badge"
import SourceLink from "./SourceLink"
import styles from "./styles.module.css"
import Tag from "./Tag"

const H3 = Heading("h3")

export default function ClassMember({
  name,
  desc,
  children,
  tags = [],
  realm = [],
  since,
  unreleased,
  deprecated,
  private: isPrivate,
  yields,
  readonly: readOnly,
  source,
  sourceUrl,
}) {
  return (
    <>
      <div className={styles.divider} />
      <H3 id={name}>
        <code
          style={{
            textDecoration: deprecated ? "line-through" : "none",
          }}
        >
          {name}
        </code>
      </H3>

      {realm.map((realm) => (
        <Badge key={realm} label={realm} />
      ))}
      {isPrivate && <Badge label="Private" />}
      {yields && <Badge label="Yields" />}
      {readOnly && <Badge label="Read Only" />}
      {tags.map((tag) => (
        <Tag key={tag} label={tag} />
      ))}

      <div className={styles.repositoryDetailsContainer}>
        {since && !deprecated && (
          <span className={styles.releaseVersion}>since {since}</span>
        )}
        {deprecated && !since && (
          <span className={styles.releaseVersion}>
            deprecated in {deprecated.version}
          </span>
        )}
        {since && deprecated && (
          <span className={styles.releaseVersion}>
            since {since}, deprecated in {deprecated.version}
          </span>
        )}
        {unreleased && !deprecated && (
          <span className={styles.releaseVersion}>unreleased</span>
        )}

        {sourceUrl && (
          <SourceLink to={`${sourceUrl}/${source.path}#L${source.line}`} />
        )}
      </div>

      {deprecated && (
        <Admonition
          variation="caution"
          title={`This was deprecated in ${deprecated.version}`}
        >
          {deprecated.desc ||
            "This item is deprecated. Do not use it for new work. "}
        </Admonition>
      )}

      {children}
    </>
  )
}
