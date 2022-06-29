import Heading from "@theme/Heading"
import clsx from "clsx"
import React from "react"
import InlineDescription from "./InlineDescription"
import LuaType from "./LuaType"
import LuaTypeDef from "./LuaTypeDef"
import Markdown from "./Markdown"
import styles from "./styles.module.css"
import { PrOp } from "./Syntax"

const Param = ({ name, lua_type }) => (
  <>
    <code>{name}:&nbsp;</code>
    <LuaType code={lua_type} />
  </>
)

export default function LuaFunction({
  luaClassName,
  name,
  params,
  returns,
  desc,
  function_type: functionType,
  errors,
  extraTypes,
}) {
  return (
    <>
      <div className={styles.memberString}>
        {name !== "__iter" ? (
          <>
            <code className={name === "__call" && styles.green}>
              {luaClassName}
              {name !== "__call" ? (functionType === "static" ? "." : ":") : ""}
            </code>
            {name !== "__call" && <code className={styles.green}>{name}</code>}
            <PrOp>(</PrOp>
            {params.length < 2 && (!params[0] || !params[0].desc) ? (
              params[0] && <Param {...params[0]} />
            ) : (
              <>
                <div className={styles.inset}>
                  {params.map((param, index) => (
                    <div key={index}>
                      <Param {...param} />
                      {index !== params.length - 1 && <code>,</code>}
                      {param.desc && <InlineDescription content={param.desc} />}
                    </div>
                  ))}
                </div>
              </>
            )}
            <PrOp>) →&nbsp;</PrOp>
            {returns.length !== 1 && <PrOp>(</PrOp>}
            {returns.length === 1 ? (
              <>
                <LuaType code={returns[0].lua_type} />
                {returns[0].desc && (
                  <InlineDescription content={returns[0].desc} />
                )}
              </>
            ) : (
              returns.length > 1 && (
                <div className={styles.inset}>
                  {returns.map((ret, index) => (
                    <div key={index}>
                      <LuaType code={ret.lua_type} />
                      {index !== returns.length - 1 && <code>,</code>}
                      {ret.desc && <InlineDescription content={ret.desc} />}
                    </div>
                  ))}
                </div>
              )
            )}
            {returns.length !== 1 && <PrOp>)</PrOp>}
          </>
        ) : (
          <>
            <code className={styles.red}>for</code>
            &nbsp;&nbsp;
            {returns.map((ret, index) => (
              <span key={index}>
                <LuaType code={ret.lua_type} />
                {index !== returns.length - 1 && <code>,</code>}
                {ret.desc && <InlineDescription content={ret.desc} />}
                &nbsp;
              </span>
            ))}
            &nbsp;
            <code className={styles.red}>in</code>
            &nbsp;&nbsp;
            <code className={styles.green}>{luaClassName}</code>
            &nbsp;&nbsp;
            <code className={styles.red}>do</code>
          </>
        )}
      </div>

      {extraTypes && (
        <>
          <Heading as="h3">Types</Heading>

          {extraTypes.map((type) => (
            <>
              <Heading as="h3" id={type.name} />
              <LuaTypeDef key={type.name} {...type} />
            </>
          ))}
        </>
      )}

      <Markdown content={desc} />

      {errors && (
        <>
          <Heading as="h3">Errors</Heading>
          <table className={clsx(styles.errorTable)}>
            <thead>
              <tr>
                <th>Type</th>
                <th>Description</th>
              </tr>
            </thead>
            <tbody>
              {errors.map((error, i) => (
                <tr key={i}>
                  <td>{error.lua_type}</td>
                  <td>{error.desc}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </>
      )}
    </>
  )
}
