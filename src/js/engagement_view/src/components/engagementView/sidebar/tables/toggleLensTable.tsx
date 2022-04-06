import React, { useEffect, useState } from "react";

import Button from "@mui/material/Button";
import KeyboardArrowDownOutlinedIcon from "@mui/icons-material/KeyboardArrowDownOutlined";
import KeyboardArrowUpOutlinedIcon from "@mui/icons-material/KeyboardArrowUpOutlined";

import Divider from "@mui/material/Divider";
import Backdrop from "@mui/material/Backdrop";
import CircularProgress from "@mui/material/CircularProgress";

import { lensTable } from "./lensTable/lensTable";
import { getLenses } from "services/graphQLRequests/getLenses";

import { ToggleLensTableProps, ToggleLensTableState } from "types/CustomTypes";

import { useStyles } from "./lensTable/lensTableStyles";

const defaultToggleLensTableState = (): ToggleLensTableState => {
    return {
        toggled: true,
        lenses: [],
        first: 100, // first is the page size
        offset: 0, // by default, start from page 0
    };
};

export function ToggleLensTable({ setLens }: ToggleLensTableProps) {
    const classes = useStyles();

    const [lensRetrievedState, setLensRetrievedState] = useState(null);
    const [toggleTableState, setToggleTableState] = useState(
        defaultToggleLensTableState()
    );
    const [pageState, setPageState] = useState(0);
    const [rowsPerPageState, setRowsPerPageState] = useState(10);

    const handleChangePage = (
        event: React.MouseEvent<HTMLButtonElement, MouseEvent> | null,
        page: number
    ) => {
        setPageState(page);
    };

    const handleChangeRowsPerPage = (
        event: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
    ) => {
        setRowsPerPageState(parseInt(event.target.value, 10));
        setPageState(0);
    };

    useEffect(() => {
        // const interval = setInterval(() => {
            getLenses(toggleTableState.first, toggleTableState.offset).then(
                (response) => {
                    if (
                        response.lenses &&
                        response.lenses !== toggleTableState.lenses
                    ) {
                        const lenses = toggleTableState.lenses.concat(
                            response.lenses
                        );
                        setLensRetrievedState(lenses as any);
                        setToggleTableState({
                            ...toggleTableState,
                            offset:
                                toggleTableState.offset +
                                    response.lenses.length || 0,
                            lenses,
                        });
                    }
                }
            );
        // }, 5000);
        // return () => clearInterval(interval);
    }, [toggleTableState]);

    return (
        <>
            <div className={classes.header}>
                <b className={classes.title}> Lenses </b>
                <Button
                    className={classes.lensToggleBtn}
                    onClick={() => {
                        setToggleTableState({
                            ...toggleTableState,
                            toggled: !toggleTableState.toggled,
                        });
                    }}
                >
                    {toggleTableState.toggled ? (
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

            <div className="lensToggle">
                {lensRetrievedState ? (
                    toggleTableState.toggled &&
                    lensTable(
                        toggleTableState,
                        pageState,
                        rowsPerPageState,
                        handleChangePage,
                        handleChangeRowsPerPage,
                        setLens,
                        classes
                    )
                ) : (
                    <Backdrop className={classes.backdrop} open>
                        <CircularProgress color="inherit" />
                    </Backdrop>
                )}
            </div>

            <Divider />
        </>
    );
}
