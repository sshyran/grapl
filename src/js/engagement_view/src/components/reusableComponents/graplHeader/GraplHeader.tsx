import React from "react";
import { Link } from "react-router-dom";

import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";

import { graplHeaderStyles } from "./styles";
import { GraplHeaderProps } from "types/GraplHeaderTypes";

import { LogoIcon } from "./LogoIcon";

const useStyles: any = graplHeaderStyles;

const GraplHeader = ({ displayBtn }: GraplHeaderProps) => {
    const classes = useStyles();

    return (
        <>
            <AppBar position="static">
                <Toolbar className={classes.header}>
                    <LogoIcon className={classes.titleIcon} />
                    <Typography variant="h6" className={classes.title}>
                        GRAPL
                    </Typography>
                    {displayBtn && (
                        <Link to="/" className={classes.link}></Link>
                    )}
                </Toolbar>
            </AppBar>
        </>
    );
};

export default GraplHeader;
