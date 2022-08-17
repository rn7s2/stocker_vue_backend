SELECT CAST(SUM(`batch`.`number`) as INTEGER) AS `number`, `batch`.`item_id`, `item`.`name`,`item`.`specification`,`item`.`unit`,`item`.`manufacturer`,`item`.`price` FROM `item` INNER JOIN `batch` ON `batch`.`item_id`=`item`.`id` WHERE `batch`.`date`>= $1 AND `batch`.`date`<= $2 GROUP BY `batch`.`item_id` ORDER BY `item`.`price` DESC