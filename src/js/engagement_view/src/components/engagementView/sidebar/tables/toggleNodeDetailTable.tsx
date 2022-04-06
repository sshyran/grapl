import React, { useState } from "react";

import Button from "@mui/material/Button";
import KeyboardArrowDownOutlinedIcon from "@mui/icons-material/KeyboardArrowDownOutlined";
import KeyboardArrowUpOutlinedIcon from "@mui/icons-material/KeyboardArrowUpOutlined";
import { ToggleNodeTableProps } from "types/LensAndNodeTableTypes";

import { NodeDetails } from "../LensAndNodeTableContainer";

import { useStyles } from "./lensTable/lensTableStyles";

export function ToggleNodeDetailTable({ curNode }: ToggleNodeTableProps) {
    const [toggled, setToggle] = useState(true);
    const classes = useStyles();
    return (
        <div>
            {curNode && (
                <div className={classes.header}>
                    <b className={classes.title}> Node Details</b>
                    <Button
                        className={classes.lensToggleBtn}
                        onClick={() => {
                            setToggle((toggled) => !toggled);
                        }}
                    >
                        {toggled === true ? (
                            <KeyboardArrowUpOutlinedIcon
                                className={classes.expand}
                            />
                        ) : (
                            <KeyboardArrowDownOutlinedIcon
                                className={classes.expand}
                            />
                        )}
                    </Button>
                </div>
            )}

            <div className="nodeToggle">
                {toggled && curNode && (
                    <div>{<NodeDetails node={curNode} />}</div>
                )}
            </div>
        </div>
    );
}
