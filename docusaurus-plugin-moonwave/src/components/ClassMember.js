import Heading from "@theme/Heading"
import React from "react"
import Admonition from "./Admonition.js"
import Badge from "./Badge.js"
import SourceLink from "./SourceLink.js"
import styles from "./styles.module.css"
import Tag from "./Tag.js"

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
  luaClassName,
}) {
  return (
    <>
      <div className={styles.divider} />
      <Heading as="h3" id={name}>
        {name === "__iter" ? (
          <span
            style={{
              fontStyle: "italic",
            }}
          >
            iterating over {luaClassName}
          </span>
        ) : (
          <code
            style={{
              textDecoration: deprecated ? "line-through" : "none",
            }}
          >
            {name === "__call" ? <>{luaClassName}()</> : name}
          </code>
        )}
      </Heading>

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
