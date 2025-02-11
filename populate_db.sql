-- psql -U ivanschuetz -d bikematch -f ./populate_db.sql

INSERT INTO bike (id, name_, brand, price, price_number, vendor_link, electric, type_, descr)
VALUES (1, 'Name 1', 'Brand 1', '999,99 €', 999.99, 'https://google.com', FALSE, 'mountain', 'Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum');


INSERT INTO bike (id, name_, brand, price, price_number, vendor_link, electric, type_, descr)
VALUES (2, 'Name 2', 'Brand 2', '2000 €', 2000, 'https://google.com', FALSE, 'hybrid', 'Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum');


INSERT INTO bike (id, name_, brand, price, price_number, vendor_link, electric, type_, descr)
VALUES (3, 'Name 3', 'Brand 3', '580 €', 580, 'https://google.com', FALSE, 'road', 'Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum');

INSERT into bike_pic(bike_id, url)
VALUES (1, 'https://picsum.photos/id/0/500/700');
INSERT into bike_pic(bike_id, url)
VALUES (1, 'https://picsum.photos/id/1/500/700');
INSERT into bike_pic(bike_id, url)
VALUES (2, 'https://picsum.photos/id/2/500/700');
INSERT into bike_pic(bike_id, url)
VALUES (2, 'https://picsum.photos/id/3/500/700');
INSERT into bike_pic(bike_id, url)
VALUES (3, 'https://picsum.photos/id/4/500/700');
INSERT into bike_pic(bike_id, url)
VALUES (3, 'https://picsum.photos/id/5/500/700');
