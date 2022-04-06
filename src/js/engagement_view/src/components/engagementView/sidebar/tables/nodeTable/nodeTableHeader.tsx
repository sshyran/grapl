import React from "react";

import TableCell from "@mui/material/TableCell";
import TableHead from "@mui/material/TableHead";
import TableRow from "@mui/material/TableRow";

import { VizNode } from "types/CustomTypes";

export function nodeTableHeader(node: VizNode, styles: any) {
    if (node) {
        return (
            <TableHead>
                <TableRow>
                    <TableCell align="left" className={styles.tableHeader}>
                        <b> Property </b>
                    </TableCell>
                    <TableCell align="right" className={styles.tableHeader}>
                        <b> Value </b>
                    </TableCell>
                </TableRow>
            </TableHead>
        );
    } else {
        return <></>;
    }
}
