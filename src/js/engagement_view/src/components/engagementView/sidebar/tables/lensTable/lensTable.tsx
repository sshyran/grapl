import React from "react";

import Table from "@mui/material/Table";
import TableBody from "@mui/material/TableBody";
import TableContainer from "@mui/material/TableContainer";
import TablePagination from "@mui/material/TablePagination";
import TableHead from "@mui/material/TableHead";
import TableRow from "@mui/material/TableRow";
import TableCell from "@mui/material/TableCell";

import { SelectLens } from "./selectLens";
import { Lens } from "types/CustomTypes";
import { PaginationState } from "types/CustomTypes";
import {ClassNameMap} from "@mui/material";

export const lensTable = (
    state: PaginationState,
    page: number,
    rowsPerPage: number,
    handleChangePage: (
        event: React.MouseEvent<HTMLButtonElement, MouseEvent> | null,
        page: number
    ) => void,
    handleChangeRowsPerPage: (
        event: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
    ) => void,
    setLens: (lens: string) => void,
    classes: ClassNameMap<string>
) => {
    return (
        <TableContainer className={classes.tableContainer}>
            <TablePagination
                className={classes.pagination}
                aria-label="pagination"
                rowsPerPageOptions={[5, 10, 25]}
                component="div"
                count={state.lenses.length}
                rowsPerPage={rowsPerPage}
                page={page}
                onPageChange={handleChangePage}
                onRowsPerPageChange={handleChangeRowsPerPage}
            />
            <Table
                className={classes.table}
                aria-label="lens-table"
                key={"lensTable"}
            >
                <TableHead>
                    <TableRow>
                        <TableCell align="left">
                            <b> Lens Name </b>
                        </TableCell>
                        <TableCell align="right">
                            <b> Risk </b>
                        </TableCell>
                    </TableRow>
                </TableHead>

                <TableBody>
                    {state.lenses
                        .slice(
                            page * rowsPerPage,
                            page * rowsPerPage + rowsPerPage
                        )
                        .map((lens: Lens) => {
                            return (
                                <SelectLens
                                    key={Number(lens.uid)}
                                    uid={lens.uid}
                                    lens={lens.lens_name}
                                    lens_type={lens.lens_type}
                                    score={lens.score}
                                    setLens={setLens}
                                />
                            );
                        })}
                </TableBody>
            </Table>
        </TableContainer>
    );
};
